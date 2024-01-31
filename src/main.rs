use std::io::Write;

use clap::Parser;
use img_caster::disk::Disk;

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

    /// Scsi number. ex) 1 -> "\\.\Scsi1"
    #[clap(short, long)]
    scsino: Option<u8>,

    /// enable to FUA mode
    #[clap(long)]
    fua: Option<bool>,
}

fn main() {
    let args = Args::parse();

    // img_caster::dev_utils::enum_dev();

    let mut filename = String::from("");
    if let Some(filepath) = args.filepath.as_deref() {
        filename = filepath.to_string();
    }
    if let Some(driveno) = args.driveno {
        let drv_c = img_caster::disk::get_physical_drv_number_from_logical_drv("C".to_string());
        if drv_c == driveno as i32 {
            println!("Can't write to system drive {driveno}");
        } else {
            filename = format!("\\\\.\\PhysicalDrive{driveno}");
        }
    }
    if let Some(scsino) = args.scsino {
        let drv_c = img_caster::disk::get_physical_drv_number_from_logical_drv("C".to_string());
        if drv_c == scsino as i32 {
            println!("Can't write to system drive {scsino}");
        } else {
            filename = format!("\\\\.\\Scsi{scsino}:");
        }
    }

    // Open file
    let mut disk = Disk::open(filename, 'w', args.fua);
    if let Some(ref mut disk) = disk {
        println!("{:?}", disk);
        // disk.storage_query_property();
        // if let Ok(scsi) = disk.get_scsi_address() {
        //     let scsi_path = format!("\\\\.\\Scsi{scsi}");
        //     println!("{:?}", scsi_path);
        // }
    }

    let mut data1 = Box::new(vec![0x33u8; 64 * 512]);
    let data2 = Box::new(vec![0x55u8; 64 * 512]);
    if let Some(ref mut disk) = disk {
        if let Err(e) = disk.write(&data1) {
            println!("Disk write Error: {:?}", e);
        }
        // if let Err(e) = disk.read(&mut data) {
        //     println!("Disk read Error: {:?}", e);
        // }
        // disk.storage_query_property();
        if let Err(e) = disk.scsi_write(&data2) {
            println!("scsi write Error: {:?}", e);
        }
        if let Err(e) = disk.scsi_read(0, &mut data1) {
            println!("scsi read Error: {:?}", e);
        }
        println!("read data {:?}", &data1[..512]);
    }
}
