use clap::Parser;
use std::fs::File;
use std::io::{Read, Result};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::thread;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
/// Sender for Multicast File Transfer
struct Args {
    /// File name to transmit data.
    #[clap(short, long, value_name = "FILE")]
    filepath: Option<String>,

    /// PhysicalDrive number. ex) 1 -> "\\.\PhysicalDrive1"
    #[clap(short, long)]
    driveno: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut filename = String::from("");

    if let Some(filepath) = args.filepath.as_deref() {
        filename = filepath.to_string();
    }

    get_interfaces();
    
    let file = File::open(filename)?;
    let (tx, rx) = mpsc::channel();
    let socket = UdpSocket::bind("127.0.0.1:12345")?;
    let sender_thread = thread::spawn(move || {
        udp_sender(socket, rx);
    });

    read_and_send(&file, tx);

    sender_thread.join().unwrap();

    Ok(())
}

fn read_and_send(mut file: &File, tx: mpsc::Sender<Vec<u8>>) {
    let mut buffer = [0; 1024];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                let data = buffer[..bytes_read].to_vec();
                println!("read data: {} {:?}", data.len(), &data[..10]);
                if tx.send(data).is_err() {
                    eprintln!("Error sending data to sender thread");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {:?}", e);
                break;
            }
        }
    }
}

fn udp_sender(socket: UdpSocket, rx: mpsc::Receiver<Vec<u8>>) {
    let dest_addr: SocketAddr = "127.0.0.1:54321".parse().unwrap();
    while let Ok(data) = rx.recv() {
        println!("send data: {} {:?}", data.len(), &data[..10]);
        if let Err(e) = socket.send_to(&data, dest_addr) {
            eprintln!("Error sending data: {:?}", e);
            break;
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
