pub mod offsets;
pub mod packets;
use crate::offsets::*;
use std::net::{Ipv4Addr, UdpSocket};

// const SIADDR: &str = "192.168.1.1";
// const YIADDR: &str = "192.168.1.100";

fn wait_for_dhcp_client(rx: &UdpSocket) -> std::io::Result<[u8; MAX_DHCPPACKET_LEN]> {
    let mut dhcp_packet = [0; MAX_DHCPPACKET_LEN];
    loop {
        let _ = rx.recv(&mut dhcp_packet)?;
        if dhcp_packet[OP_OCTET] == 1 {
            break;
        }
        dhcp_packet = [0; MAX_DHCPPACKET_LEN];
    }
    Ok(dhcp_packet)
}

fn cook() -> std::io::Result<()> {
    let rx = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 67))?;
    let tx = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 68))?;

    tx.set_broadcast(true).unwrap();

    let siaddr: String = dotenvy::var("SIADDR").unwrap();
    let yiaddr: String = dotenvy::var("YIADDR").unwrap();
    let limited_broadcast: String = dotenvy::var("IP_LIMITED_BROADCAST").unwrap();

    {
        // Wait for a DHCPDISCOVER
        let dhcpdiscover_p = wait_for_dhcp_client(&rx)?;
        println!("Length: {}", dhcpdiscover_p.len());
        // println!("{:#x?}", dhcpdiscover_p);
        // Retrieve data from DHCPDISCOVER
        let xid = <[u8; 4]>::try_from(&dhcpdiscover_p[XID_START..XID_END]).unwrap();
        let chaddr = <[u8; 16]>::try_from(&dhcpdiscover_p[CH_MAC_START..CH_MAC_END]).unwrap();
        let flags = <[u8; 2]>::try_from(&dhcpdiscover_p[FLAGS..FLAGS + 2]).unwrap();
        let giaddr = <[u8; 4]>::try_from(&dhcpdiscover_p[GIADDR_START..GIADDR_START + 4]).unwrap();
        println!("Received DHCPDISCOVER! xid: {:#x}", u32::from_be_bytes(xid));
        println!("Byte0 xid: {:#x}", xid[0]);
        println!("mac: {:#x?}", chaddr);

        // Send DHCPOFFER
        packets::DHCP::from(xid, &yiaddr, &siaddr, chaddr, giaddr, flags)
            .set_type(packets::DHCPOFFER)
            .set_default_options(&siaddr, &limited_broadcast)
            .generate_and_send(&tx)?;
    }

    {
        println!("Waiting for DHCPREQUEST");
        // Wait for a DHCPREQUEST
        let dhcprequest_p = wait_for_dhcp_client(&rx)?;
        // Retrieve data from DHCPREQUEST
        let xid = <[u8; 4]>::try_from(&dhcprequest_p[XID_START..XID_END]).unwrap();
        let chaddr = <[u8; 16]>::try_from(&dhcprequest_p[CH_MAC_START..CH_MAC_END]).unwrap();
        let flags = <[u8; 2]>::try_from(&dhcprequest_p[FLAGS..FLAGS + 2]).unwrap();
        let giaddr = <[u8; 4]>::try_from(&dhcprequest_p[GIADDR_START..GIADDR_START + 4]).unwrap();
        println!("Received DHCPREQUEST! xid: {:#x}", u32::from_be_bytes(xid));

        // Send DHCPACK
        packets::DHCP::from(xid, &yiaddr, &siaddr, chaddr, giaddr, flags)
            .set_type(packets::DHCPACK)
            .set_default_options(&siaddr, &limited_broadcast)
            .generate_and_send(&tx)?;
    }

    println!("Acknowledged!");

    Ok(())
}

fn main() {
    dotenvy::dotenv().unwrap();
    cook().unwrap();
}
