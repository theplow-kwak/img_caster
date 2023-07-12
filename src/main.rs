mod disk;
mod multicast;

fn main() {
    // Specify the physical drive path
    let filename = "\\\\.\\PhysicalDrive0";
    let rwflag = 'r';
    // Open the physical drive with appropriate options
    let mut disk = disk::DiskHandler::new(filename.to_string(), rwflag);

    let mut buffer = [0u8; 1024];

    disk.open();
    disk.read(&mut buffer).expect("Failed to read from physical drive");
    disk.read(&mut buffer).expect("Failed to read from physical drive");
    disk.read(&mut buffer).expect("Failed to read from physical drive");
}
