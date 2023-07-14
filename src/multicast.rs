use std::io;
use std::net::{UdpSocket, SocketAddrV4, SocketAddr, Ipv4Addr};

pub struct MultiCast {
    socket: UdpSocket,
    multicast_addr: Ipv4Addr,
    multicast_group: SocketAddrV4,
}

impl MultiCast {
    pub fn receiver() -> MultiCast {
        // Create a UDP socket
        let socket = UdpSocket::bind("0.0.0.0:9000").expect("Failed to bind socket");
    
        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);

        // Join the multicast group
        socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))
            .expect("Failed to join multicast group");

        MultiCast {socket, multicast_addr, multicast_group}
    }
       
    pub fn sender() -> MultiCast {
        // Create a UDP socket
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    
        // Set the TTL (time to live) for multicast packets
        socket.set_multicast_ttl_v4(2).expect("Failed to set TTL");
    
        let multicast_addr = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_group = SocketAddrV4::new(multicast_addr, 9000);

        // Join the multicast group
        socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))
            .expect("Failed to join multicast group");

        MultiCast {socket, multicast_addr, multicast_group}
    }
    
    pub fn send_msg(&mut self, message: &[u8]) {
        println!("Sent multicast message: '{:?}'", message);
        self.socket.send_to(message, self.multicast_group)
            .expect("Failed to send multicast packet");
    }

    pub fn recv_msg(&mut self, buf: &mut [u8]) -> (usize, SocketAddr) {
        let (size, src_addr) = self.socket.recv_from(buf).expect("Failed to receive multicast packet");
        println!("Receive multicast message: '{:?}'", &buf[..size]);
        (size, src_addr)
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }
}
