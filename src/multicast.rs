use socket2::{Domain, Socket, Type};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

pub struct MultiCast {
    socket: UdpSocket,
    multicast_addr: Ipv4Addr,
    multicast_group: SocketAddrV4,
}

impl MultiCast {
    pub fn receiver() -> MultiCast {
        // let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        // socket.set_recv_buffer_size(1024 * 1024).unwrap(); // Set the receive buffer size to 1MB
        // let address: SocketAddr = "[::1]:0".parse().unwrap();
        // println!("{:?}", address);
        // let address = address.into();
        // println!("{:?}", address);
        // socket.bind(&address);
        // let udp_socket = UdpSocket::from(socket);

        // Create a UDP socket
        let socket = UdpSocket::bind("0.0.0.0:9000").expect("Failed to bind socket");
        socket.set_nonblocking(true).unwrap();

        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);

        // Join the multicast group
        socket
            .join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))
            .expect("Failed to join multicast group");

        MultiCast {
            socket,
            multicast_addr,
            multicast_group,
        }
    }

    pub fn sender() -> MultiCast {
        // Create a UDP socket
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

        // Set the TTL (time to live) for multicast packets
        socket.set_multicast_ttl_v4(2).expect("Failed to set TTL");

        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);

        // Join the multicast group
        socket
            .join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))
            .expect("Failed to join multicast group");

        MultiCast {
            socket,
            multicast_addr,
            multicast_group,
        }
    }

    pub fn send_msg(&mut self, message: &[u8]) {
        println!("Sent to {}: '{:?}'\n", self.multicast_group, message);
        self.socket
            .send_to(message, self.multicast_group)
            .expect("Failed to send multicast packet");
    }

    pub fn recv_msg(&mut self, buf: &mut [u8]) -> (usize, SocketAddr) {
        // let (size, src_addr) = self.socket.recv_from(buf).expect("Failed to receive multicast packet");
        // println!("Receive multicast message: '{:?}'", &buf[..size]);
        loop {
            match self.socket.recv_from(buf) {
                Ok((size, address)) => return (size, address),
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    // Handle timeout or perform other tasks
                    // ...
                }
                Err(err) => {
                    // Handle other errors
                    // ...
                }
            }
        }
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }
}

// socket.set_nonblocking(true).expect("Failed to set non-blocking mode");

// loop {
//     let mut buffer = [0u8; 1024];
//     match socket.recv_from(&mut buffer) {
//         Ok((size, _)) => {
//             // Process received data
//             // ...
//         }
//         Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
//             // No data received yet, do other tasks or sleep for a while
//             std::thread::sleep(Duration::from_millis(10));
//         }
//         Err(err) => {
//             // Handle error
//             println!("Error: {}", err);
//             break;
//         }
//     }
// }

// socket.set_nonblocking(true)?;

// let mut buffer = [0; 1024];
// let timeout_duration = Duration::from_secs(1); // Set a timeout of 1 second

// loop {
//     match socket.recv_from(&mut buffer) {
//         Ok((size, address)) => {
//             // Process received data
//             // ...
//         }
//         Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
//             // Handle timeout or perform other tasks
//             // ...
//         }
//         Err(err) => {
//             // Handle other errors
//             // ...
//         }
//     }
// }

fn rcvbuffer() {
    use socket2::{SockAddr, Socket, Type};
    use std::net::UdpSocket;

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    socket.set_recv_buffer_size(1024 * 1024).unwrap(); // Set the receive buffer size to 1MB
    let udp_socket = UdpSocket::from(socket);
}

fn nonblocking() {
    use std::time::Duration;

    let socket = UdpSocket::bind("0.0.0.0:12345").unwrap();
    socket.set_nonblocking(true).unwrap();

    let mut buffer = [0; 1024];
    let timeout_duration = Duration::from_secs(1); // Set a timeout of 1 second

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, address)) => {
                // Process received data
                // ...
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                // Handle timeout or perform other tasks
                // ...
            }
            Err(err) => {
                // Handle other errors
                // ...
            }
        }
    }
}
