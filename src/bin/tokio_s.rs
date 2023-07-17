use std::net::{SocketAddr, UdpSocket};
use tokio::io::Result;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio::time::Duration;

async fn send_multicast_message(addr: SocketAddr, message: &[u8]) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let tokio_socket = TokioUdpSocket::from_std(socket)?;

    loop {
        tokio_socket.send_to(message, addr).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let multicast_addr: SocketAddr = "239.0.0.1:9000".parse().unwrap();
    let message = b"Hello, multicast!";

    let send_task = send_multicast_message(multicast_addr, message);

    tokio::try_join!(send_task)?;
    Ok(())
}
