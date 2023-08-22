use byte_unit::Byte;
use clap::Parser;
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::*;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;

use img_caster::datafifo::DataFIFO;
use img_caster::disk::Disk;
use img_caster::receiver;
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
    #[clap(short, long)]
    chunk: Option<String>,

    /// Receive buffer size.
    #[clap(short, long, default_value = "8MiB")]
    rcvbuf: Option<String>,

    /// set to Async file write
    #[clap(short, long)]
    async_mode: bool,

    /// Log file name
    #[clap(short, long)]
    log: Option<String>,

    #[clap(long, default_value = "info")]
    loglevel: Option<String>,
}

fn write(disk: &mut Option<Disk>, data_fifo: Arc<Mutex<DataFIFO>>, write_chunk: usize) -> bool {
    loop {
        // {
        let mut data_fifo = data_fifo.lock().unwrap();
        let mut required: usize = data_fifo.len();
        if !data_fifo.close && ((required % write_chunk) != 0) {
            required -= required % write_chunk;
        }
        if required > 0 {
            if let Some(data) = data_fifo.pop(required) {
                if let Some(ref mut disk) = disk {
                    let mut iter = data.chunks(write_chunk);
                    while let Some(data) = iter.next() {
                        let _n = disk.write(&data);
                    }
                }
            }
        }
        if data_fifo.close {
            return false;
        }
        // }
        // thread::sleep(Duration::from_nanos(10));
    }
}

fn init_logger(args: &Args) {
    // initialize logger
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

    // Open file
    let mut disk = Disk::open(filename.to_string(), 'w');
    if let Some(ref d) = disk {
        info!("{:?}", d);
    }

    let mut write_chunk = WRITE_CHUNK;
    if let Some(chunk) = args.chunk {
        write_chunk = Byte::from_str(chunk).unwrap().get_bytes() as usize * SECTOR_SIZE;
    }

    let data_fifo = Arc::new(Mutex::new(DataFIFO::new()));
    let data_fifo_thread = Arc::clone(&data_fifo);
    let data_fifo_socket = Arc::clone(&data_fifo);

    let rcvbuf = Byte::from_str(args.rcvbuf.unwrap()).unwrap().get_bytes() as usize;
    let mut receiver =
        receiver::McastReceiver::new(args.nic.unwrap_or(0), rcvbuf, data_fifo_socket);

    let disk_thread = thread::spawn(move || {
        write(&mut disk, data_fifo_thread, write_chunk);
    });

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
                    let _ = receiver.send_disconnect();
                    break;
                }
            }
        }
    }
    receiver.display_progress(true);
    data_fifo.lock().unwrap().close = true;
    let _ = disk_thread.join();
}
