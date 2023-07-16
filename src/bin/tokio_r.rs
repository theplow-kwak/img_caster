use std::net::{SocketAddr, Ipv4Addr, UdpSocket};
use tokio::io::Result;
use tokio::net::UdpSocket as TokioUdpSocket;

async fn receive_multicast_messages(addr: Ipv4Addr) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    // let tokio_socket = TokioUdpSocket::from_std(socket)?;
    // let my_ip: Ipv4Addr = "0.0.0.0".parse().unwrap();
    // let socket = TokioUdpSocket::bind(my_ip).await?;
    let tokio_socket = TokioUdpSocket::from_std(socket)?;
    tokio_socket.join_multicast_v4(addr, Ipv4Addr::UNSPECIFIED)?;

    let mut buffer = [0u8; 1024];

    loop {
        let (size, _) = tokio_socket.recv_from(&mut buffer).await?;
        let message = &buffer[..size];
        
        // Process the received message
        println!("Received message: {:?}", message);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let multicast_addr: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let message = b"Hello, multicast!";

    let receive_task = receive_multicast_messages(multicast_addr);

    tokio::try_join!(receive_task)?;
    Ok(())
}
