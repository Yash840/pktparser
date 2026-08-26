use std::fmt;
use std::fmt::Formatter;
use crate::ether::EtherType::{IPv4, ARP, IPv6, UNKNOWN};
use crate::ether::ParseError::Truncated;
const ETHERTYPE_IPV4: u16 = 0x0800;
const ETHERTYPE_ARP: u16 = 0x0806;
const ETHERTYPE_IPV6: u16 = 0x86DD;
#[derive(Debug)]
pub enum ParseError {
    Truncated {
        needed: usize,
        available: usize
    }
}

pub enum EtherType{
    IPv4,
    ARP,
    IPv6,
    UNKNOWN
}

impl EtherType {
    pub fn from(code: u16) -> EtherType {
        match code {
            ETHERTYPE_IPV4 => IPv4,
            ETHERTYPE_ARP => ARP,
            ETHERTYPE_IPV6 => IPv6,
            _ => UNKNOWN
        }
    }

    pub fn to_string(self: &Self) -> String {
        match self {
            IPv4 => "IPv4".to_string(),
            ARP => "ARP".to_string(),
            IPv6 => "IPv6".to_string(),
            _ => "UNKNOWN".to_string()
        }
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
pub struct MacAddr([u8; 6]);

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
               self.0[0], self.0[1], self.0[2],
               self.0[3], self.0[4], self.0[5])
    }
}
pub struct EtherFrame<'a> {
    pub source: MacAddr,
    pub destination: MacAddr,
    pub ether_type: EtherType,
    pub payload: &'a [u8],
}

impl fmt::Display for EtherFrame<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{source: {}, destination: {}, ether_type: {}, payload: {:?}}}",
               self.source, self.destination, self.ether_type, self.payload)
    }
}
pub struct Ether;

impl Ether {
    pub fn parse(buffer: &[u8]) -> Result<EtherFrame<'_>, ParseError> {
        if buffer.len() < 14 {
            return Err(Truncated {
                needed: 14,
                available: buffer.len()
            });
        }

        let destination: [u8; 6] = buffer[0..6].try_into().unwrap();
        let source: [u8; 6] = buffer[6..12].try_into().unwrap();

        let type_code = u16::from_be_bytes([buffer[12], buffer[13]]);

        let frame = EtherFrame {
            source: MacAddr(source),
            destination: MacAddr(destination),
            ether_type: EtherType::from(type_code),
            payload: &buffer[14..],
        };

        Ok(frame)
    }
}