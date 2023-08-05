use log::{debug, error, info, trace, warn};
use socket2::{Domain, SockAddr, Socket, Type};
use std::env;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::process::Command;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::WINDOWS_949; // Windows-949는 한글 인코딩입니다.

pub struct MultiCast {
    socket: UdpSocket,
    multicast_addr: Ipv4Addr,
    multicast_group: SocketAddrV4,
}

#[derive(Debug)]
pub struct NetworkInterface {
    ip_address: Option<String>,
    subnet_mask: Option<String>,
}

fn convert_to_english(input: &str) -> String {
    let encoded_bytes = WINDOWS_949.encode(input, EncoderTrap::Replace).unwrap();
    String::from_utf8_lossy(&encoded_bytes).to_string()
}

pub fn parse_ipconfig_output() -> Vec<NetworkInterface> {
    let output = Command::new("ipconfig")
        .env("LANG", "en-US") // Set LANG environment variable to "en-US"
        .output()
        .expect("Failed to execute ipconfig.");

    let output = String::from_utf8_lossy(&output.stdout);
    // info!("{}", output);
    // let english_output = convert_to_english(&output);
    // info!("{}", english_output);

    let mut interfaces: Vec<NetworkInterface> = Vec::new();
    let mut current_interface: Option<NetworkInterface> = None;

    for line in output.lines() {
        info!("{}", line);
        if line.starts_with("Ethernet adapter") {
            current_interface = Some(NetworkInterface {
                ip_address: None,
                subnet_mask: None,
            });
        } else if let Some(ref mut interface) = current_interface {
            if line.trim().starts_with("IPv4 Address") {
                let parts: Vec<&str> = line.trim().split(':').collect();
                if let Some(ip_str) = parts.get(1) {
                    interface.ip_address = Some(ip_str.trim().to_string());
                }
            } else if line.trim().starts_with("Subnet Mask") {
                let parts: Vec<&str> = line.trim().split(':').collect();
                if let Some(subnet_mask_str) = parts.get(1) {
                    interface.subnet_mask = Some(subnet_mask_str.trim().to_string());
                }
            } else if line.trim().is_empty() {
                if let Some(interface) = current_interface.take() {
                    interfaces.push(interface);
                }
            }
        }
    }

    interfaces
}

impl MultiCast {
    pub fn receiver() -> MultiCast {
        // Create a socket
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        socket.set_nonblocking(true).unwrap();

        let rcvbuf_size = 4096;
        socket.set_recv_buffer_size(rcvbuf_size);

        let addr: SocketAddr = "0.0.0.0:9000".parse().unwrap();
        socket.bind(&addr.into());

        let socket: UdpSocket = socket.into();
        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);
        let mut sockaddr_multicast = SockAddr::from(multicast_group);

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
        // let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        let socket: UdpSocket = socket.into();

        // Set the TTL (time to live) for multicast packets
        socket.set_multicast_ttl_v4(2).expect("Failed to set TTL");
        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);
        let mut sockaddr_multicast = SockAddr::from(multicast_group);

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
        // println!("Sent to {:?}: '{:?}'\n", self.multicast_group, message);
        self.socket
            .send_to(message, &self.multicast_group)
            .expect("Failed to send multicast packet");
    }

    pub fn recv_msg(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
        // let (size, src_addr) = self.socket.recv_from(buf).expect("Failed to receive multicast packet");
        // println!("Receive multicast message: '{:?}'", &buf[..size]);
        // loop {
        //     match self.socket.recv_from(buf) {
        //         Ok((size, address)) => return (size, address),
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
