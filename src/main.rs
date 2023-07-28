use std::error::Error;
use std::net::UdpSocket;
use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a UDP socket using the socket2 crate
    let domain = Domain::IPV4;
    let socket_type = Type::DGRAM;
    let socket = Socket::new(domain, socket_type, None)?;

    // Set the receive buffer size (rcvbuf size) to 4096 bytes
    let rcvbuf_size = 4096;
    socket.set_recv_buffer_size(rcvbuf_size);

    // Bind the socket to an address (e.g., 127.0.0.1:8080)
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    socket.bind(&addr.into())?;

    // Convert the socket into a UdpSocket
    let udp_socket: UdpSocket = socket.into();

    // Now, you can use udp_socket to send and receive UDP packets
    // ...

    Ok(())
}
