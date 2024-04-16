use std::fmt;

pub static TCP_VERSION: u8 = 1;

pub struct TcpPacket {
    pub version: u8,
    pub stat_code: u8,
    pub command: TcpPacketCommand,
    pub length: u16,
    pub data: Option<String>,
}

pub enum FromOption {
    Packet(TcpPacket),
    Status(u8),
    None
}

#[allow(dead_code)]
pub enum TcpPacketCommand {
    None,
    Health,
    Save,
    Get
}

pub fn get_command_string(packet: &TcpPacketCommand) -> String {
    match packet {
        TcpPacketCommand::Health => "Health".into(),
        TcpPacketCommand::Save => "Save XML".into(),
        TcpPacketCommand::Get => "Get XML".into(),
        TcpPacketCommand::None => "No Command".into(),
    }
}

pub fn get_command_num(packet: &TcpPacketCommand) -> u8 {
    match packet {
        TcpPacketCommand::None => 0,
        TcpPacketCommand::Health => 1,
        TcpPacketCommand::Save => 2,
        TcpPacketCommand::Get => 3
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
            stat_code: 100,
            length: 0,
            data: Some(String::new()),
        }
    }

    pub fn from(buf: &[u8]) -> FromOption {
        if buf.len() == 0 {
            return FromOption::None;
        }

        if buf[1] < 20 || buf[1] > 29 {
            return FromOption::Status(buf[1]);
        }

        let len: [u8; 2] = [buf[3], buf[4]];

        let command: TcpPacketCommand = match buf[2] {
            1 => TcpPacketCommand::Health,
            2 => TcpPacketCommand::Save,
            3 => TcpPacketCommand::Get,
            _ => TcpPacketCommand::None,
        };

        FromOption::Packet(TcpPacket {
            version: buf[0],
            stat_code: buf[1],
            command,
            length: u16::from_le_bytes(len),
            data: Some(String::from_utf8(buf[5..].to_vec()).unwrap()),
        })
    }

    pub fn command(command: TcpPacketCommand) -> Self {
        TcpPacket {
            version: TCP_VERSION,
            stat_code: 20,
            command,
            length: 0,
            data: None
        }
    }

    pub fn data(command: TcpPacketCommand, data: String) -> Self {
        TcpPacket {
            version: TCP_VERSION,
            stat_code: 20,
            command,
            length: data.len() as u16,
            data: Some(data)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.push(self.version);
        buf.push(self.stat_code);
        buf.push(get_command_num(&self.command));

        buf.extend_from_slice(&self.length.to_le_bytes());

        let d: &[u8] = match &self.data {
            Some(v) => {
                v.as_bytes()
            },
            None => &[]
        };

        buf.extend_from_slice(d);
        return buf;
    }

    pub fn add_success(&mut self) -> &Self {
        self.stat_code = 20;
        self
    }

    pub fn add_failure(&mut self) -> &Self {
        self.stat_code = 40;
        self
    }
}

impl fmt::Display for TcpPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = get_command_string(&self.command);

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
