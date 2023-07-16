use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::Result;
use tokio::net::UdpSocket as TokioUdpSocket;

async fn receive_multicast_messages(addr: SocketAddr) -> Result<()> {
    let my_ip: Ipv4Addr = "0.0.0.0".parse().unwrap();
    let socket = TokioUdpSocket::bind(my_ip).await?;
    socket.join_multicast_v4(addr, Ipv4Addr::UNSPECIFIED)?;

    let mut buffer = [0u8; 1024];

    loop {
        let (size, _) = socket.recv_from(&mut buffer).await?;
        let message = &buffer[..size];
        
        // Process the received message
        println!("Received message: {:?}", message);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let multicast_addr: SocketAddr = "239.0.0.1:12345".parse().unwrap();
    let message = b"Hello, multicast!";

    let receive_task = receive_multicast_messages(multicast_addr);

    tokio::try_join!(receive_task)?;
    Ok(())
}
