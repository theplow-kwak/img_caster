use crate::packet::Message;
use crate::*;

use ipnet::Ipv4Net;
use log::{debug, error, info, trace, warn};
use socket2::{Domain, Socket, Type};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::process::Command;
use std::time::Duration;

#[derive(Debug)]
struct NetworkInterface {
    ip_address: Option<String>,
    subnet_mask: Option<String>,
}

impl From<NetworkInterface> for Ipv4Net {
    fn from(interface: NetworkInterface) -> Ipv4Net {
        Ipv4Net::with_netmask(
            interface
                .ip_address
                .expect("Can't get ip address")
                .parse()
                .expect("Unable to parse socket address"),
            interface
                .subnet_mask
                .expect("Can't get subnet mask")
                .parse()
                .expect("Unable to parse socket address"),
        )
        .expect("Unable to parse socket address")
    }
}

fn get_ipconfig() -> Vec<Ipv4Net> {
    let mut interfaces: Vec<Ipv4Net> = Vec::new();
    let mut current_interface: Option<NetworkInterface> = None;
    let mut is_taken = false;

    let output = Command::new("ipconfig")
        .output()
        .expect("Failed to execute ipconfig");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
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
                    is_taken = true;
                }
            } else if line.trim().starts_with("Subnet Mask") {
                let parts: Vec<&str> = line.trim().split(':').collect();
                if let Some(subnet_mask_str) = parts.get(1) {
                    interface.subnet_mask = Some(subnet_mask_str.trim().to_string());
                }
            } else if line.trim().is_empty() && is_taken {
                if let Some(interface) = current_interface.take() {
                    interfaces.push(Ipv4Net::from(interface));
                    is_taken = false;
                }
            }
        }
    }

    if let Some(interface) = current_interface.take() {
        interfaces.push(Ipv4Net::from(interface));
    }

    interfaces
}

#[derive(Debug)]
pub struct MultiCast {
    socket: UdpSocket,
    pub myip_addr: SocketAddrV4,
    pub broadcast_addr: SocketAddrV4,
    pub multicast_addr: SocketAddrV4,
    pub receivefrom: Option<SocketAddrV4>,
    pub packet_count: usize,
}

impl MultiCast {
    pub fn receiver(nic: usize, rcvbuf: usize) -> Self {
        let myip_net = get_default_interface(); // get_ipconfig()[nic];
        let myip_addr: SocketAddrV4 = SocketAddrV4::new(myip_net.addr(), PORTBASE);

        let broadcast_addr = SocketAddrV4::new(myip_net.broadcast(), PORTBASE + 1);
        let multicast_addr = Ipv4Addr::from(u32::from(myip_net.addr()) & 0x07ffffff | 0xe8000000);
        let multicast_addr = SocketAddrV4::new(multicast_addr, PORTBASE + 1);

        // Create a UDP socket
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        let _ = socket.set_recv_buffer_size(rcvbuf);
        let _ = socket.bind(&myip_addr.into());

        let socket: UdpSocket = socket.into();
        let _ = socket.set_read_timeout(Some(Duration::from_millis(50)));

        Self {
            socket,
            myip_addr,
            broadcast_addr,
            multicast_addr,
            receivefrom: None,
            packet_count: 0,
        }
    }

    pub fn sender(nic: usize) -> Self {
        let myip_net = get_default_interface(); // get_ipconfig()[nic];
        let myip_addr: SocketAddrV4 = SocketAddrV4::new(myip_net.addr(), PORTBASE + 1);

        let broadcast_addr = SocketAddrV4::new(myip_net.broadcast(), PORTBASE);
        let multicast_addr = Ipv4Addr::from(u32::from(myip_net.addr()) & 0x07ffffff | 0xe8000000);
        let multicast_addr = SocketAddrV4::new(multicast_addr, PORTBASE);

        // Create a UDP socket
        let socket = UdpSocket::bind(myip_addr.to_string()).expect("Failed to bind socket");
        let _ = socket.set_read_timeout(Some(Duration::from_millis(10)));

        Self {
            socket,
            myip_addr,
            broadcast_addr,
            multicast_addr,
            receivefrom: None,
            packet_count: 0,
        }
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.socket.set_multicast_ttl_v4(ttl)
    }

    pub fn set_broadcast(&self) -> io::Result<()> {
        self.socket.set_broadcast(true)
    }

    pub fn join_multicast(&self) -> io::Result<()> {
        self.socket
            .join_multicast_v4(&self.multicast_addr.ip(), &self.myip_addr.ip())
    }

    pub fn set_nonblocking(&mut self) -> io::Result<()> {
        self.socket.set_nonblocking(true)
    }

    pub fn send_msg(&mut self, message: &[u8]) -> io::Result<usize> {
        self.packet_count += 1;
        if let Some(rcvfrom) = self.receivefrom {
            self.socket.send_to(message, rcvfrom)
        } else {
            self.socket.send_to(message, self.multicast_addr)
        }
    }

    pub fn send_to(&mut self, message: &[u8], sendto: SocketAddrV4) -> io::Result<usize> {
        self.packet_count += 1;
        self.socket.send_to(message, sendto)
    }

    pub fn recv_msg(&mut self, buf: &mut [u8]) -> io::Result<(Message, Vec<u8>)> {
        self.packet_count += 1;
        match self.socket.recv_from(buf) {
            Ok((size, address)) => {
                let (msg, remain) = Message::decode(&buf[..size]);
                match address {
                    SocketAddr::V4(v4) => self.receivefrom = Some(v4),
                    _ => self.receivefrom = None,
                }
                trace!("message {:?} from {address}", msg);
                return Ok((msg, remain));
            }
            Err(err) => return Err(err),
        }
    }
}

pub fn get_interfaces() {
    let interfaces = default_net::get_interfaces();
    for interface in interfaces {
        println!("Interface");
        println!("\tIndex: {}", interface.index);
        println!("\tName: {}", interface.name);
        println!("\tFriendly Name: {:?}", interface.friendly_name);
        println!("\tDescription: {:?}", interface.description);
        println!("\tType: {}", interface.if_type.name());
        if let Some(mac_addr) = interface.mac_addr {
            println!("\tMAC: {}", mac_addr);
        } else {
            println!("\tMAC: (Failed to get mac address)");
        }
        println!("\tIPv4: {:?}", interface.ipv4);
        println!("\tIPv6: {:?}", interface.ipv6);
        println!("\tFlags: {:?}", interface.flags);
        println!("\tTransmit Speed: {:?}", interface.transmit_speed);
        println!("\tReceive Speed: {:?}", interface.receive_speed);
        if let Some(gateway) = interface.gateway {
            println!("Gateway");
            println!("\tMAC: {}", gateway.mac_addr);
            println!("\tIP: {}", gateway.ip_addr);
        } else {
            println!("Gateway: (Not found)");
        }
        println!();
    }
}

pub fn get_default_interface() -> Ipv4Net {
    match default_net::get_default_interface() {
        Ok(default_interface) => {
            return Ipv4Net::new(
                default_interface.ipv4[0].addr,
                default_interface.ipv4[0].prefix_len,
            )
            .expect("can't get default ip");
        }
        Err(e) => {
            println!("{}", e);
            Ipv4Net::default()
        }
    }
}
