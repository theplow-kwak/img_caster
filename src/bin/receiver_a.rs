use byte_unit::Byte;
use clap::Parser;
use log::{info, LevelFilter};
use simplelog::*;
use std::fs::File;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

use img_caster::datafifo::DataFIFO;
use img_caster::disk::Disk;
use img_caster::receiver_a::{McastReceiver, write};
use img_caster::*;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
/// Receiver for Multicast File Transfer
struct Args {
    /// File name to save the received data.
    #[clap(short, long, value_name = "FILE")]
    filepath: Option<String>,

    /// PhysicalDrive number. ex) 1 -> "\\.\PhysicalDrive1"
    #[clap(short, long)]
    driveno: Option<u8>,

    /// Specifie the network card.
    #[clap(short, long, default_value = "0")]
    nic: Option<usize>,

    /// Number of sectors to set Write chunk size.
    #[clap(short, long, default_value = "512")]
    chunk: Option<String>,

    /// Log file name
    #[clap(short, long)]
    log: Option<String>,

    #[clap(long, default_value = "info")]
    loglevel: Option<String>,

    #[clap(long, default_value = "512MiB")]
    pipesize: Option<String>,

    /// Receive buffer size.
    #[clap(long, default_value = "8MiB")]
    rcvbuf: Option<String>,
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
    println!("Img_Caster: receiver v{}\n", VERSION);

    // Open file
    let mut disk = Disk::open(filename.to_string(), 'w');
    if let Some(ref d) = disk {
        info!("{:?}", d);
    }

    let data_fifo = Arc::new(RwLock::new(DataFIFO::new(MAX_BUFFER_SIZE)));
    let data_fifo_thread = Arc::clone(&data_fifo);
    let data_fifo_socket = Arc::clone(&data_fifo);

    let disk_trace: Arc<RwLock<Box<Vec<(Instant, Instant)>>>> =
        Arc::new(RwLock::new(Box::new(Vec::new())));
    let disk_trace_thread = Arc::clone(&disk_trace);

    // Open Network socket receiver
    let write_chunk = Byte::from_str(args.chunk.clone().unwrap())
        .unwrap()
        .get_bytes() as usize
        * SECTOR_SIZE;
    let rcvbuf = Byte::from_str(args.rcvbuf.unwrap()).unwrap().get_bytes() as usize;
    let pipesize = Byte::from_str(args.pipesize.clone().unwrap())
        .unwrap()
        .get_bytes() as usize;
    let mut receiver = McastReceiver::new(args.nic.unwrap_or(0), rcvbuf, data_fifo_socket);
    receiver.set_pipesize(pipesize);

    let disk_thread =
        thread::spawn(move || write(&mut disk, data_fifo_thread, write_chunk, disk_trace_thread));

    let _ = receiver.enumerate();

    println!("\nPress 'Enter' to start receiving data!\n");

    loop {
        if let Ok(running) = receiver.dispatch_message() {
            if !running {
                break;
            }
        }
        if !receiver.transferstarted {
            if let Some(c) = getch(0) {
                if c == '\r' {
                    receiver.start_transfer();
                }
                if c == 'q' {
                    break;
                }
            }
        }
        if data_fifo.read().unwrap().is_closed() {
            break;
        }
    }
    let _ = receiver.send_disconnect();
    let _ = disk_thread.join();
    receiver.display_progress(true);

    let filename = format!(
        "ar{}_{}_{}_{}.csv",
        receiver.id(),
        receiver.socket.myip_addr.ip().to_string(),
        args.chunk.unwrap(),
        args.pipesize.unwrap()
    );
    let mut events = receiver.get_events();
    for (start_time, end_time) in disk_trace.write().unwrap().iter() {
        events.push(("disk".to_owned(), *start_time, *end_time));
    }
    save_trace(&filename, events, receiver.start_time);
}
