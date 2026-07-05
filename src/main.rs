pub mod offsets;
pub mod packets;
use crate::{offsets::*, packets::DHCPType};
use std::net::{Ipv4Addr, UdpSocket};

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

    tx.set_broadcast(true)?;

    let siaddr: String = dotenvy::var("SIADDR").expect("Unset SIADDR variable");
    let yiaddr: String = dotenvy::var("YIADDR").expect("Unset YIADDR variable");
    let limited_broadcast: String =
        dotenvy::var("IP_LIMITED_BROADCAST").expect("Unset IP_LIMITED_BROADCAST variable");

    {
        // Wait for a DHCPDISCOVER
        println!("Waiting for DHCPDISCOVER");
        let dhcpdiscover_p = wait_for_dhcp_client(&rx)?;
        // Retrieve data from DHCPDISCOVER
        let xid = <[u8; 4]>::try_from(&dhcpdiscover_p[XID_START..XID_END]).unwrap();
        let chaddr = <[u8; 16]>::try_from(&dhcpdiscover_p[CH_MAC_START..CH_MAC_END]).unwrap();
        let flags = <[u8; 2]>::try_from(&dhcpdiscover_p[FLAGS..FLAGS + 2]).unwrap();
        let giaddr = <[u8; 4]>::try_from(&dhcpdiscover_p[GIADDR_START..GIADDR_START + 4]).unwrap();
        println!("Received DHCPDISCOVER! xid: {:#x}", u32::from_be_bytes(xid));

        // Send DHCPOFFER
        packets::DHCP::from(
            xid,
            &yiaddr,
            &siaddr,
            chaddr,
            giaddr,
            flags,
            DHCPType::DHCPOffer,
        )
        .set_default_options(&siaddr, &limited_broadcast)
        .generate_and_send(&tx)?;
    }

    {
        // Wait for a DHCPREQUEST
        println!("Waiting for DHCPREQUEST");
        let dhcprequest_p = wait_for_dhcp_client(&rx)?;
        // Retrieve data from DHCPREQUEST
        let xid = <[u8; 4]>::try_from(&dhcprequest_p[XID_START..XID_END]).unwrap();
        let chaddr = <[u8; 16]>::try_from(&dhcprequest_p[CH_MAC_START..CH_MAC_END]).unwrap();
        let flags = <[u8; 2]>::try_from(&dhcprequest_p[FLAGS..FLAGS + 2]).unwrap();
        let giaddr = <[u8; 4]>::try_from(&dhcprequest_p[GIADDR_START..GIADDR_START + 4]).unwrap();
        println!("Received DHCPREQUEST! xid: {:#x}", u32::from_be_bytes(xid));

        // Send DHCPACK
        packets::DHCP::from(
            xid,
            &yiaddr,
            &siaddr,
            chaddr,
            giaddr,
            flags,
            DHCPType::DHCPAck,
        )
        .set_default_options(&siaddr, &limited_broadcast)
        .generate_and_send(&tx)?;
    }

    println!("Acknowledged! Success.");

    Ok(())
}

fn main() {
    dotenvy::dotenv().unwrap();
    cook().unwrap();
}
