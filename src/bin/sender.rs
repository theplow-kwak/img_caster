use byte_unit::Byte;
use clap::Parser;
use log::{debug, error, info, trace, warn, LevelFilter};
use simplelog::*;
use std::fs::File;
use std::str::FromStr;
use std::time::Duration;

use img_caster::sender::McastSender;

#[derive(Parser, Default, Debug, Clone)]
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

    let mut transfer_size = 0;
    if let Some(size) = args.size {
        transfer_size = Byte::from_str(size).unwrap().get_bytes() as usize;
    }

    // Open Network socket sender
    let mut sender = McastSender::new(
        args.nic.unwrap_or(0),
        &filename,
        transfer_size,
        args.ttl.unwrap(),
        Byte::from_str(args.slices.unwrap()).unwrap().get_bytes() as u32,
    );

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
    sender.display_progress(true);
}
