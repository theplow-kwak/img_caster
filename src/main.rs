use clap::Parser;

use img_caster::disk::Disk;

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

    // Open file
    let mut disk = Disk::open(filename.to_string(), 'w');
    if let Some(ref d) = disk {
        println!("{:?}", d);
    }


}
