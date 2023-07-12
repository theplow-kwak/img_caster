use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;


fn main() {
    // Create a UDP socket
    let mut receiver = MultiCast::receiver();

    // Receive multicast packets
    let mut buf = [0u8; 1024];
    let (size, src_addr) = receiver.recv_from(&mut buf).expect("Failed to receive multicast packet");

    // let message = String::from_utf8_lossy(&buf[..size]);
    // println!("Received multicast message from {}: '{}'", src_addr, message);

    let (size, src_addr) = receiver.recv_msg(&mut buf);

    // let message = String::from_utf8_lossy(&buf[..size]);
    // println!("Received multicast message from {}: '{}'", src_addr, message);
}
