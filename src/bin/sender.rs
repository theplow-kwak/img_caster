use byte_unit::Byte;
use clap::Parser;
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::*;
use std::fs::File;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use img_caster::disk::Disk;
use img_caster::sender::McastSender;

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

    /// set to Async file read
    #[clap(short, long)]
    async_mode: bool,

    /// enable to p2p mode
    #[clap(short, long)]
    p2p: bool,

    /// Log file name
    #[clap(short, long)]
    log: Option<String>,

    #[clap(long, default_value = "info")]
    loglevel: Option<String>,
}

struct Sender {
    sock: McastSender,
    disk: Option<Disk>,
    logger: Vec<Box<dyn SharedLogger>>,
}

fn read_and_send(mut file: &File, tx: mpsc::Sender<Vec<u8>>) {
    let mut buffer = [0; 1024];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                let data = buffer[..bytes_read].to_vec();
                println!("read data: {} {:?}", data.len(), &data[..10]);
                if tx.send(data).is_err() {
                    eprintln!("Error sending data to sender thread");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {:?}", e);
                break;
            }
        }
    }
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

    // initialize logger
    let loglevel = args.loglevel.unwrap();
    let termlog = TermLogger::new(
        LevelFilter::from_str(&loglevel).unwrap(),
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    let mut logger: Vec<Box<dyn SharedLogger>> = vec![termlog];

    if let Some(logfile) = args.log {
        let flog = WriteLogger::new(
            LevelFilter::from_str(&loglevel).unwrap(),
            Config::default(),
            File::create(logfile).unwrap(),
        );
        logger.push(flog);
    }
    let _ = CombinedLogger::init(logger);

    let mut transfer_size = 0;
    if let Some(size) = args.size {
        transfer_size = Byte::from_str(size).unwrap().get_bytes() as usize;
    }

    // Open file
    let rwflag = 'r';
    let mut disk = Disk::open(filename.to_string(), rwflag);
    if let Some(ref mut disk) = disk {
        if transfer_size > 0 {
            disk.size = transfer_size;
        }
        info!("{:?}", disk);
    }

    let (tx, rx) = mpsc::channel();

    // Open Network socket sender
    let mut sender = McastSender::new(
        args.nic.unwrap_or(0),
        args.ttl.unwrap(),
        Byte::from_str(args.slices.unwrap()).unwrap().get_bytes() as u32,
        rx,
    );
    if args.async_mode {
        let _ = sender.socket.set_nonblocking();
    }

    if let Err(err) = sender.enumerate(Duration::new(args.wait.unwrap_or(60 * 5), 0), args.p2p) {
        error!("{:?}", err);
        return;
    }

    // let sender = Sender {
    //     sock: sender,
    //     disk,
    //     logger,
    // };

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
    sender.display_progress(true);
}
