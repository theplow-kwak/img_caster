use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;

fn main() {
    // Create a UDP socket
    let mut sender = MultiCast::sender();

    let filename = "\\\\.\\PhysicalDrive0";
    let rwflag = 'r';
    // Open the physical drive with appropriate options
    let mut disk = DiskHandler::new(filename.to_string(), rwflag);
    let mut buffer = [0u8; 1024];

    disk.open();
    disk.read(&mut buffer).expect("Failed to read from physical drive");

    let message = "Hello, multicast!";
    sender.send_msg(&buffer);
    sender.send_msg(message.as_bytes());
}
