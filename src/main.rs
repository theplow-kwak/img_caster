use log::{debug, error, info, trace, warn};
use simplelog::*;
use std::fs::File;

use img_caster::multicast;
use img_caster::bitarray::BitArray;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    multicast::parse_ipconfig_output();

    trace!("Commencing yak shaving");
    info!("Razor located: {}", 2);
    warn!("Unable to locate a razor: {}, retrying", 3);

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

    match default_net::get_default_interface() {
        Ok(default_interface) => {
            println!("Default Interface");
            println!("\tIndex: {}", default_interface.index);
            println!("\tName: {}", default_interface.name);
            println!("\tFriendly Name: {:?}", default_interface.friendly_name);
            println!("\tDescription: {:?}", default_interface.description);
            println!("\tType: {}", default_interface.if_type.name());
            if let Some(mac_addr) = default_interface.mac_addr {
                println!("\tMAC: {}", mac_addr);
            } else {
                println!("\tMAC: (Failed to get mac address)");
            }
            println!("\tIPv4: {:?}", default_interface.ipv4);
            println!("\tIPv6: {:?}", default_interface.ipv6);
            println!("\tFlags: {:?}", default_interface.flags);
            println!("\tTransmit Speed: {:?}", default_interface.transmit_speed);
            println!("\tReceive Speed: {:?}", default_interface.receive_speed);
            if let Some(gateway) = default_interface.gateway {
                println!("Default Gateway");
                println!("\tMAC: {}", gateway.mac_addr);
                println!("\tIP: {}", gateway.ip_addr);
            } else {
                println!("Default Gateway: (Not found)");
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    if let Some(hostname) = system_info::HostName::get() {
        println!("host name: {}", hostname);
    }

    let mut buf = [
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 4, 8, 9, 10, 11, 12, 13, 14, 15,
        16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]; // From PackedSize

    let mut vbuf = buf.to_vec();
    let mut map = BitArray::new(100);

    map.set(4);
    map.set(14);
    map.set(44);
    map.set(54);
    println!("map {:?}", map);   

    let mut bits = map.bits();
    vbuf.append(&mut bits);
    println!("append bits to vec buf {:?}", &vbuf);
    println!("map {:?}", map);   
    
    let mut map2 = BitArray::new(100);
    map2.set(74);
    map2.set(53);
    map2.set(22);
    map2.set(54);
    println!("map2 {:?}", map2);   

    map |= map2;
    println!("map |= map2 {:?}", map);   

}