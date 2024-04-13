use std::fmt;

pub struct TcpPacket {
    pub version: u8,
    pub command: Option<TcpPacketCommand>,
    pub length: u16,
    pub data: Option<String>
}

pub enum TcpPacketCommand {
    Health
}

impl fmt::Display for TcpPacketCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TcpPacketCommand::Health => "Health"
        })
    }
}

#[allow(dead_code)]
impl TcpPacket {
    pub fn new() -> Self {
        TcpPacket {
            version: 1,
            command: None,
            length: 0,
            data: Some(String::new())
        }
    }

    pub fn from(buf: &[u8]) -> Self {
        let len: [u8; 2] = [buf[2], buf[3]];

        let command: Option<TcpPacketCommand> = match buf[1] {
            1 => Some(TcpPacketCommand::Health),
            _ => None
        };

        TcpPacket {
            version: buf[0],
            command,
            length: u16::from_le_bytes(len),
            data: Some(String::from_utf8(buf[4..].to_vec()).unwrap())
        }
    }
}

impl fmt::Display for TcpPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let health = match &self.command {
            Some(v) => match v {
                TcpPacketCommand::Health => "Health"
            },
            None => "No Command"
        };

        let data: String = match &self.data {
            Some(v) => v.to_string(),
            None => "No Data".to_string()
        };

        write!(f, "Version: {}, Command: {}, Length: {}, Data: {}",
               self.version,
               health,
               self.length,
               data)
    }
}
