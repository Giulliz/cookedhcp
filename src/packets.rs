use crate::offsets::*;
use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
    str::FromStr,
};

pub const DHCPOFFER: u8 = 2;
pub const DHCPACK: u8 = 5;
pub const DHCPNAK: u8 = 6;

pub struct DHCP {}
pub struct DHCPTyper {
    dhcp: Vec<u8>,
}
pub struct DHCPOptioner {
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
    ) -> DHCPTyper {
        let ip_yiaddr = Ipv4Addr::from_str(yiaddr).unwrap().octets();
        let ip_siaddr = Ipv4Addr::from_str(siaddr).unwrap().octets();

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
            dhcp[CIADDR_START + i] = 0;
        }
        for i in 0..4 {
            dhcp[YIADDR_START + i] = ip_yiaddr[i];
        }
        for i in 0..4 {
            dhcp[SIADDR_START + i] = ip_siaddr[i];
        }
        for i in 0..4 {
            dhcp[GIADDR_START + i] = giaddr[i];
        }
        for i in 0..16 {
            dhcp[CH_MAC_START + i] = chaddr[i];
        }
        // Zeroed options already set
        let magic_cookie = u32::to_be_bytes(0x63825363);
        for i in 0..4 {
            dhcp[MAGIC_COOKIE + i] = magic_cookie[i];
        }

        DHCPTyper {
            dhcp: dhcp.to_vec(),
        }
    }
}

impl DHCPTyper {
    pub fn set_type(&mut self, dhcp_type: u8) -> DHCPOptioner {
        // ident53
        self.dhcp.push(53);
        self.dhcp.push(1);
        self.dhcp.push(dhcp_type);

        DHCPOptioner {
            dhcp: self.dhcp.clone(),
        }
    }
}

impl DHCPOptioner {
    pub fn add_option(&mut self, option_number: u8, len: u8, data: Vec<u8>) -> DHCPOptioner {
        self.dhcp.push(option_number);
        self.dhcp.push(len);
        data.into_iter().for_each(|b| self.dhcp.push(b));

        DHCPOptioner {
            dhcp: self.dhcp.clone(),
        }
    }

    pub fn set_default_options(
        &mut self,
        siaddr: &String,
        limited_broadcast_ip: &String,
    ) -> DHCPOptioner {
        let dhcp = &mut self.dhcp;
        let ip_siaddr = Ipv4Addr::from_str(siaddr).unwrap().octets();
        let ip_dns = dotenvy::var("IP_DNS").expect(
            "IP_DNS variable not found. 
        Make sure to set it in a .env file in the root project directory.",
        );

        // type 54
        dhcp.push(54);
        dhcp.push(4);
        for i in 0..4 {
            dhcp.push(ip_siaddr[i]);
        }
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
        Ipv4Addr::from_str(limited_broadcast_ip)
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
        // dns6
        if ip_dns != "NO" {
            dhcp.push(6);
            dhcp.push(4);
            Ipv4Addr::from_str(&ip_dns)
                .unwrap()
                .octets()
                .into_iter()
                .for_each(|b| dhcp.push(b));
            // end255
            dhcp.push(255);
        }

        DHCPOptioner { dhcp: dhcp.clone() }
    }

    pub fn generate_and_send(&self, tx: &UdpSocket) -> io::Result<usize> {
        tx.send_to(self.dhcp.clone().as_slice(), "255.255.255.255:68")
    }
}
