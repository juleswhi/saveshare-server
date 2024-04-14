use std::fmt;

pub static TCP_VERSION: u8 = 1;

pub struct TcpPacket {
    pub version: u8,
    pub command: TcpPacketCommand,
    pub length: u16,
    pub data: Option<String>,
}

#[allow(dead_code)]
pub enum TcpPacketCommand {
    None,
    Health,
    SaveXML
}

pub fn get_command_string(packet: &TcpPacketCommand) -> String {
    match packet {
        TcpPacketCommand::Health => "Health".to_string(),
        TcpPacketCommand::SaveXML => "Save XML".to_string(),
        TcpPacketCommand::None => "No Command".to_string(),
    }
}

pub fn get_command_num(packet: &TcpPacketCommand) -> u8 {
    match packet {
        TcpPacketCommand::Health => 1,
        _ => 0
    }
}

impl fmt::Display for TcpPacketCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            get_command_string(self)
        )
    }
}

#[allow(dead_code)]
impl TcpPacket {
    pub fn new() -> Self {
        TcpPacket {
            version: 1,
            command: TcpPacketCommand::None,
            length: 0,
            data: Some(String::new()),
        }
    }

    pub fn from(buf: &[u8]) -> Self {
        let len: [u8; 2] = [buf[2], buf[3]];

        let command: TcpPacketCommand = match buf[1] {
            1 => TcpPacketCommand::Health,
            _ => TcpPacketCommand::None,
        };

        TcpPacket {
            version: buf[0],
            command,
            length: u16::from_le_bytes(len),
            data: Some(String::from_utf8(buf[4..].to_vec()).unwrap()),
        }
    }

    pub fn command(command: TcpPacketCommand) -> Self {
        TcpPacket {
            version: TCP_VERSION,
            command,
            length: 0,
            data: None
        }
    }

    pub fn data(command: TcpPacketCommand, data: String) -> Self {
        TcpPacket {
            version: TCP_VERSION,
            command,
            length: data.len() as u16,
            data: Some(data)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.push(self.version);
        buf.push(get_command_num(&self.command));

        buf.extend_from_slice(&self.length.to_le_bytes());

        let d: &[u8] = match &self.data {
            Some(v) => {
                v.as_bytes()
            },
            None => &[]
        };

        buf.extend_from_slice(d);
        buf
    }
}

impl fmt::Display for TcpPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = match self.command {
            TcpPacketCommand::Health => "Health",
            TcpPacketCommand::SaveXML => "SaveXML",
            TcpPacketCommand::None => "No Command",
        };

        let data: String = match &self.data {
            Some(v) => v.to_string(),
            None => "No Data".to_string(),
        };

        write!(
            f,
            "Version: {}, Command: {}, Length: {}, Data: {}",
            self.version, cmd, self.length, data
        )
    }
}
