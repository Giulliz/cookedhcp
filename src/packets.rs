use std::{net::Ipv4Addr, str::FromStr};

use crate::offsets::*;

// #[repr(C)]
// pub struct DHCPOFFER {
//     pub op: u8,
//     pub htype: u8,
//     pub hlen: u8,
//     pub hops: u8,
//     pub xid: u32,
//     pub secs: u16,
//     pub flags: u16,
//     pub ciaddr: u32,
//     pub yiaddr: u32,
//     pub siaddr: u32,
//     pub giaddr: u32,
//     pub chaddr: [u8; 6],
//     _hw_non_mac_padding: PhantomData<[u8; 10]>,
//     _bootp_legacy: PhantomData<[u8; 192]>, // 192 bytes padding or dhcp options
//     pub magic_cookie: u32,
// }

// impl DHCPOFFER {
// }

pub const DHCPOFFER: u8 = 2;
pub const DHCPACK: u8 = 5;
pub const DHCPNAK: u8 = 6;

pub fn dhcp_from(
    xid: [u8; 4],
    yiaddr: &str,
    siaddr: &str,
    chaddr: [u8; 16],
    giaddr: [u8; 4],
    flags: [u8; 2],
    dhcptype: u8,
) -> [u8; SEND_DHCPPACKET_LEN] {
    let ip_yiaddr = Ipv4Addr::from_str(yiaddr).unwrap().octets();
    let ip_siaddr = Ipv4Addr::from_str(siaddr).unwrap().octets();

    // reset values following
    let mut dhcp = [0; SEND_DHCPPACKET_LEN - OPTIONS];
    dhcp[OP_OCTET] = 2;
    dhcp[HTYPE_OCTET] = 1;
    dhcp[HLEN_OCTET] = 6;
    dhcp[HOPS_OCTET] = 0;
    for (i, &byte) in xid.iter().enumerate() {
        dhcp[XID_START + i] = byte;
    }
    dhcp[SECS] = 0;
    dhcp[SECS + 1] = 0;
    dhcp[FLAGS] = flags[0];
    dhcp[FLAGS + 1] = flags[1];
    for i in 0..4 {
        dhcp[CIADDR + i] = 0;
    }
    for i in 0..4 {
        dhcp[YIADDR + i] = ip_yiaddr[i];
    }
    for i in 0..4 {
        dhcp[SIADDR + i] = ip_siaddr[i];
    }
    for i in 0..4 {
        dhcp[GIADDR + i] = giaddr[i];
    }
    for i in 0..16 {
        dhcp[CH_MAC_START + i] = chaddr[i];
    }
    // Zeroed options already set
    // TODO SET DNS!
    let magic_cookie = u32::to_be_bytes(0x63825363);
    for i in 0..4 {
        dhcp[MAGIC_COOKIE + i] = magic_cookie[i];
    }

    let mut dhcp = dhcp.to_vec();
    // println!("Inizio len: {}", dhcp.len());
    // type 54
    dhcp.push(54);
    dhcp.push(4);
    for i in 0..4 {
        dhcp.push(ip_siaddr[i]);
    }
    // ident53
    dhcp.push(53);
    dhcp.push(1);
    dhcp.push(dhcptype);
    // lease51
    dhcp.push(51);
    dhcp.push(4);
    u32::to_be_bytes(3600)
        .into_iter()
        .for_each(|b| dhcp.push(b));
    // renewal58
    dhcp.push(58);
    dhcp.push(4);
    u32::to_be_bytes(1800)
        .into_iter()
        .for_each(|b| dhcp.push(b));
    // rebind59
    dhcp.push(59);
    dhcp.push(4);
    u32::to_be_bytes(3150)
        .into_iter()
        .for_each(|b| dhcp.push(b));
    // subnetmask1
    dhcp.push(1);
    dhcp.push(4);
    for _ in 0..3 {
        dhcp.push(255);
    }
    dhcp.push(0);
    // broadcast28
    dhcp.push(28);
    dhcp.push(4);
    Ipv4Addr::from_str("192.168.1.255")
        .unwrap()
        .octets()
        .into_iter()
        .for_each(|b| dhcp.push(b));
    // router3
    dhcp.push(3);
    dhcp.push(4);
    for i in 0..4 {
        dhcp.push(ip_siaddr[i]);
    }
    // end255
    dhcp.push(255);

    // println!("Length: {}", dhcp.len());

    <[u8; SEND_DHCPPACKET_LEN]>::try_from(dhcp).unwrap()
}
