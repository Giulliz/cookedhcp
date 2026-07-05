use crate::offsets::*;
use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
    str::FromStr,
};

const DHCPOFFER: u8 = 2;
const DHCPACK: u8 = 5;
const DHCPNAK: u8 = 6;
const DHCP_MAGIC: u32 = 0x63825363;
const DEFAULT_DNS: &str = "1.1.1.1";

pub enum DHCPType {
    DHCPOffer,
    DHCPAck,
    DHCPNak,
}

impl DHCPType {
    fn number(self) -> u8 {
        match self {
            DHCPType::DHCPOffer => DHCPOFFER,
            DHCPType::DHCPAck => DHCPACK,
            DHCPType::DHCPNak => DHCPNAK,
        }
    }
}

pub struct DHCP {
    dhcp: Vec<u8>,
}

impl DHCP {
    pub fn from(
        xid: [u8; 4],
        yiaddr: &String,
        siaddr: &String,
        chaddr: [u8; 16],
        giaddr: [u8; 4],
        flags: [u8; 2],
        dhcp_type: DHCPType,
    ) -> DHCP {
        let ip_yiaddr = Ipv4Addr::from_str(yiaddr)
            .expect("Malformed YIADDR")
            .octets();
        let ip_siaddr = Ipv4Addr::from_str(siaddr)
            .expect("Malformed SIADDR")
            .octets();

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
        dhcp[CIADDR_START..(4 + CIADDR_START)].clone_from_slice(&[0; 4]);
        dhcp[YIADDR_START..(4 + YIADDR_START)].clone_from_slice(&ip_yiaddr[..]);
        dhcp[SIADDR_START..(4 + SIADDR_START)].clone_from_slice(&ip_siaddr[..]);
        dhcp[GIADDR_START..(4 + GIADDR_START)].clone_from_slice(&giaddr[..]);
        dhcp[CH_MAC_START..(16 + CH_MAC_START)].clone_from_slice(&chaddr[..]);
        // Zeroed options already set
        let magic_cookie = u32::to_be_bytes(DHCP_MAGIC);
        dhcp[MAGIC_COOKIE..(4 + MAGIC_COOKIE)].clone_from_slice(&magic_cookie[..]);

        let mut dhcp = dhcp.to_vec();
        // set type; ident53
        dhcp.push(53);
        dhcp.push(1);
        dhcp.push(dhcp_type.number());

        DHCP { dhcp }
    }

    pub fn add_option(mut self, option_number: u8, len: u8, data: Vec<u8>) -> DHCP {
        self.dhcp.push(option_number);
        self.dhcp.push(len);
        data.into_iter().for_each(|b| self.dhcp.push(b));

        DHCP { dhcp: self.dhcp }
    }

    pub fn set_default_options(mut self, siaddr: &String, limited_broadcast_ip: &String) -> DHCP {
        let ip_siaddr = Ipv4Addr::from_str(siaddr)
            .expect("Malformed SIADDR")
            .octets();
        let ip_dns = dotenvy::var("IP_DNS").ok();

        // type 54
        self.dhcp.push(54);
        self.dhcp.push(4);
        for i in 0..4 {
            self.dhcp.push(ip_siaddr[i]);
        }
        // lease51
        self.dhcp.push(51);
        self.dhcp.push(4);
        u32::to_be_bytes(3600)
            .into_iter()
            .for_each(|b| self.dhcp.push(b));
        // renewal58
        self.dhcp.push(58);
        self.dhcp.push(4);
        u32::to_be_bytes(1800)
            .into_iter()
            .for_each(|b| self.dhcp.push(b));
        // rebind59
        self.dhcp.push(59);
        self.dhcp.push(4);
        u32::to_be_bytes(3150)
            .into_iter()
            .for_each(|b| self.dhcp.push(b));
        // subnetmask1
        self.dhcp.push(1);
        self.dhcp.push(4);
        for _ in 0..3 {
            self.dhcp.push(255);
        }
        self.dhcp.push(0);
        // broadcast28
        self.dhcp.push(28);
        self.dhcp.push(4);
        Ipv4Addr::from_str(limited_broadcast_ip)
            .unwrap()
            .octets()
            .into_iter()
            .for_each(|b| self.dhcp.push(b));
        // router3
        self.dhcp.push(3);
        self.dhcp.push(4);
        for i in 0..4 {
            self.dhcp.push(ip_siaddr[i]);
        }
        // dns6
        self.dhcp.push(6);
        self.dhcp.push(4);
        let ip_dns = match ip_dns {
            Some(ip) => ip,
            None => String::from(DEFAULT_DNS),
        };
        Ipv4Addr::from_str(&ip_dns)
            .unwrap()
            .octets()
            .into_iter()
            .for_each(|b| self.dhcp.push(b));

        // end255
        self.dhcp.push(255);

        DHCP { dhcp: self.dhcp }
    }

    // caller must allow broadcast to the tx socket
    pub fn generate_and_send(self, tx: &UdpSocket) -> io::Result<usize> {
        tx.send_to(self.dhcp.as_slice(), "255.255.255.255:68")
    }
}
