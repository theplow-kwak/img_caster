use byte_unit::Byte;
use clap::Parser;
use log::{error, info, trace, LevelFilter};
use simplelog::*;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use img_caster::datafifo::DataFIFO;
use img_caster::disk::Disk;
use img_caster::sender::McastSender;
use img_caster::*;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
/// Sender for Multicast File Transfer
struct Args {
    /// File name to transmit data.
    #[clap(short, long, value_name = "FILE")]
    filepath: Option<String>,

    /// PhysicalDrive number. ex) 1 -> "\\.\PhysicalDrive1"
    #[clap(short, long)]
    driveno: Option<u8>,

    /// Specifie the network card.
    #[clap(short, long, default_value = "0")]
    nic: Option<usize>,

    /// Set TTL value
    #[clap(short, long, default_value = "1")]
    ttl: Option<u32>,

    /// Wait time(seconds) to start transmit
    #[clap(short, long)]
    wait: Option<u64>,

    /// Transfer size. ex) 100MB, 100MiB, 205KiB
    #[clap(short, long)]
    size: Option<String>,

    /// Specifie the slice size under 8192. ex) 2048, 4KiB,
    #[clap(long, default_value = "2048")]
    slices: Option<String>,

    /// enable to p2p connection
    #[clap(short, long)]
    p2p: bool,

    /// Number of sectors to set read chunk size.
    #[clap(short, long, default_value = "8192")]
    chunk: Option<String>,

    /// Log file name
    #[clap(short, long)]
    log: Option<String>,

    #[clap(long, default_value = "info")]
    loglevel: Option<String>,

    /// enable to FUA mode
    #[clap(long)]
    fua: Option<bool>,
}

fn read(
    disk: &mut Option<Disk>,
    data_fifo: Arc<RwLock<DataFIFO>>,
    read_chunk: usize,
    disk_trace: Arc<RwLock<Box<Vec<(Instant, Instant)>>>>,
) -> bool {
    loop {
        let delay = Instant::now() + Duration::from_millis(50);
        let mut size: usize = MAX_BUFFER_SIZE - data_fifo.read().unwrap().len();
        if (size % read_chunk) != 0 {
            size -= size % read_chunk;
        }
        if let Some(ref mut disk) = disk {
            if data_fifo.read().unwrap().endpoint() >= disk.size {
                data_fifo.write().unwrap().close();
                trace!("read end");
                return false;
            }
            let start = Instant::now();
            let mut buff = Box::new(vec![0u8; size]);
            if let Ok(size) = disk.read(&mut buff) {
                trace!("read {size} bytes");
                if size > 0 {
                    data_fifo.write().unwrap().push(&mut buff[..size]);
                    let end = Instant::now();
                    disk_trace.write().unwrap().push((start, end));
                }
            }
        }
        if data_fifo.read().unwrap().is_closed() {
            return false;
        }
        thread::sleep(delay.saturating_duration_since(Instant::now()));
    }
}

// initialize logger
fn init_logger(args: &Args) {
    let loglevel = args.loglevel.as_ref().unwrap();
    let termlog = TermLogger::new(
        LevelFilter::from_str(&loglevel).unwrap(),
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    let mut logger: Vec<Box<dyn SharedLogger>> = vec![termlog];

    if let Some(logfile) = args.log.as_ref() {
        let flog = WriteLogger::new(
            LevelFilter::from_str(&loglevel).unwrap(),
            Config::default(),
            File::create(logfile).unwrap(),
        );
        logger.push(flog);
    }
    let _ = CombinedLogger::init(logger);
}

fn main() {
    let args = Args::parse();

    let mut filename = String::from("");
    if let Some(filepath) = args.filepath.as_deref() {
        filename = filepath.to_string();
    }
    if let Some(driveno) = args.driveno {
        filename = format!("\\\\.\\PhysicalDrive{driveno}");
    }

    init_logger(&args);
    println!("Img_Caster: sender v{}\n", VERSION);

    let read_chunk = Byte::from_str(args.chunk.clone().unwrap())
        .unwrap()
        .get_bytes() as usize
        * SECTOR_SIZE;

    let mut transfer_size = 0;
    if let Some(size) = args.size {
        transfer_size = Byte::from_str(size).unwrap().get_bytes() as usize;
    }

    // Open file
    let mut disk = Disk::open(filename.to_string(), 'r', args.fua);
    if let Some(ref mut disk) = disk {
        if transfer_size > 0 {
            disk.size = transfer_size;
        }
        info!("{:?}", disk);
    }

    let data_fifo = Arc::new(RwLock::new(DataFIFO::new(MAX_BUFFER_SIZE)));
    let data_fifo_thread = Arc::clone(&data_fifo);
    let data_fifo_socket = Arc::clone(&data_fifo);

    let disk_trace: Arc<RwLock<Box<Vec<(Instant, Instant)>>>> =
        Arc::new(RwLock::new(Box::new(Vec::new())));
    let disk_trace_thread = Arc::clone(&disk_trace);

    // Open Network socket sender
    let mut sender = McastSender::new(
        args.nic.unwrap_or(0),
        args.ttl.unwrap(),
        Byte::from_str(args.slices.clone().unwrap())
            .unwrap()
            .get_bytes() as u32,
        data_fifo_socket,
    );
    let disk_thread =
        thread::spawn(move || read(&mut disk, data_fifo_thread, read_chunk, disk_trace_thread));
    // thread::sleep(Duration::from_secs(2));

    if let Err(err) = sender.enumerate(Duration::new(args.wait.unwrap_or(60 * 5), 0), args.p2p) {
        error!("{:?}", err);
        return;
    }

    loop {
        if !sender.transfer_data() {
            break;
        }
        if let Ok(running) = sender.dispatch_message() {
            if !running {
                break;
            }
        }
    }
    data_fifo.write().unwrap().close();
    let _ = disk_thread.join();
    sender.display_progress(true);

    let filename = format!(
        "as{}_{}.csv",
        sender.socket.myip_addr.ip().to_string(),
        args.slices.unwrap()
    );
    let mut events = sender.get_events();
    for (start_time, end_time) in disk_trace.write().unwrap().iter() {
        events.push(("disk".to_owned(), *start_time, *end_time));
    }
    save_trace(&filename, events, sender.start_time);
}
