use clap::Parser;
use img_caster::dev_utils::NvmeControllerList;
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
    driveno: Option<i32>,

    /// pci bus number. ex) 3 -> "3:0.0"
    #[clap(short, long)]
    busno: Option<i32>,

    /// Scsi number. ex) 1 -> "\\.\Scsi1"
    #[clap(short, long)]
    scsino: Option<i32>,

    /// enable to FUA mode
    #[clap(long)]
    fua: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let mut controller_list = NvmeControllerList::new();
    controller_list.enumerate();
    println!("{}", controller_list);

    let mut filename = String::from("");
    if let Some(filepath) = args.filepath.as_deref() {
        filename = filepath.to_string();
    }
    if let Some(driveno) = args.driveno {
        filename = controller_list.by_num(driveno).unwrap_or("".into());
    }
    if let Some(busno) = args.busno {
        filename = controller_list.by_bus(busno).unwrap_or("".into());
    }
    if let Some(scsino) = args.scsino {
        let drv_c = img_caster::disk::get_physical_drv_number_from_logical_drv("C:".to_string());
        if drv_c == scsino {
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

    // let mut data1 = Box::new(vec![0x33u8; 64 * 512]);
    // let data2 = Box::new(vec![0x55u8; 64 * 512]);
    if let Some(ref mut _disk) = disk {
        // if let Err(e) = disk.write(&data1) {
        //     println!("Disk write Error: {:?}", e);
        // }
        // if let Err(e) = disk.read(&mut data) {
        //     println!("Disk read Error: {:?}", e);
        // }
        // if let Err(e) = disk.discovery0() {
        //     println!("discovery0 Error: {:?}", e);
        // }
        // if let Err(e) = disk.scsi_write(&data2) {
        //     println!("scsi write Error: {:?}", e);
        // }
        // if let Err(e) = disk.scsi_read(0, &mut data1) {
        //     println!("scsi read Error: {:?}", e);
        // }
        // println!("read data {:?}", &data1[..512]);
    }
}
