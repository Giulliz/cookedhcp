pub const PAYLOAD_OFFSET: usize = 28;
const BUFFER_OPTION: usize = 300;
pub const MAX_DHCPPACKET_LEN: usize = 264 + 4 - PAYLOAD_OFFSET + BUFFER_OPTION;

pub const TYPE54: usize = 4 + 2;
pub const IDENT53: usize = 1 + 2;
pub const LEASE51: usize = 4 + 2;
pub const RENEWAL58: usize = 4 + 2;
pub const REBINDING59: usize = 4 + 2;
pub const SUBNETMASK1: usize = 4 + 2;
pub const BROADCAST28: usize = 4 + 2;
pub const ROUTER3: usize = 4 + 2;
pub const DNS6: usize = 4 + 2;
pub const END255: usize = 1;

pub const OPTIONS: usize = TYPE54
    + IDENT53
    + LEASE51
    + RENEWAL58
    + REBINDING59
    + SUBNETMASK1
    + BROADCAST28
    + ROUTER3
    + DNS6
    + END255;
pub const SEND_DHCPPACKET_LEN: usize = 240 + OPTIONS;

pub const OP_OCTET: usize = 28 - PAYLOAD_OFFSET;
pub const HTYPE_OCTET: usize = 29 - PAYLOAD_OFFSET;
pub const HLEN_OCTET: usize = 30 - PAYLOAD_OFFSET;
pub const HOPS_OCTET: usize = 31 - PAYLOAD_OFFSET;
pub const XID_START: usize = 32 - PAYLOAD_OFFSET;
pub const XID_END: usize = 36 - PAYLOAD_OFFSET;
pub const SECS: usize = 37 - PAYLOAD_OFFSET;
pub const FLAGS: usize = 39 - PAYLOAD_OFFSET;
pub const CIADDR: usize = 40 - PAYLOAD_OFFSET;
pub const YIADDR: usize = 44 - PAYLOAD_OFFSET;
pub const SIADDR: usize = 48 - PAYLOAD_OFFSET;
pub const GIADDR: usize = 52 - PAYLOAD_OFFSET;
pub const CH_MAC_START: usize = 56 - PAYLOAD_OFFSET;
pub const CH_MAC_END: usize = CH_MAC_START + 16;
pub const MAGIC_COOKIE: usize = 264 - PAYLOAD_OFFSET;
pub const OPTIONS_START: usize = 240;
