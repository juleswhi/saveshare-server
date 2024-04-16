use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    config::Config,
    logger::{info, log, warn, LogType},
    packet::{
        FromOption, TcpPacket,
        TcpPacketCommand::{self, Health},
    },
};

pub struct Server {
    pub ip: String,
    pub port: u32,
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Self {
        let conf = Config::get_from_file().expect("Could not find config.json");
        Server {
            ip: conf.ip,
            port: conf.port,
        }
    }

    pub fn from(ip: String, port: u32) -> Self {
        Server { port, ip }
    }

    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn start_server(&self) {
        let listener: TcpListener;
        match TcpListener::bind(self.get_ip()) {
            Ok(v) => listener = v,
            Err(err) => {
                println!("{}", err);
                log(LogType::Panic);
                return;
            }
        }

        info(&format!("Server starting on {}", self.get_ip()).to_string());

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_conn(stream);
        }
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let received: &[u8];

    match buf_reader.fill_buf() {
        Ok(v) => received = v,
        Err(_) => {
            warn("Could not unwrap a buffer.");
            return;
        }
    }

    let mut packet: TcpPacket = TcpPacket::new();

    match TcpPacket::from(received) {
        FromOption::Packet(v) => packet = v,
        FromOption::Status(s) => warn(format!("Invalid status code from client: {}", s).as_str()),
        _ => warn("Request with invalid tcp"),
    };

    handle_request(packet, &mut stream);
}

fn handle_request(packet: TcpPacket, stream: &mut TcpStream) {
    match packet.command {
        TcpPacketCommand::Health => handle_health(packet, stream),
        TcpPacketCommand::Save => handle_save(packet, stream),
        TcpPacketCommand::Get => handle_get(packet, stream),
        _ => {}
    }
}

fn handle_health(_: TcpPacket, stream: &mut TcpStream) {
    let cmd = TcpPacket::command(Health).to_bytes();
    let buf: &[u8] = &cmd.as_slice();

    let res = stream.write_all(buf);

    match res {
        Ok(_) => {}
        Err(_) => warn("data did not send succesfully"),
    }
}

fn handle_save(_: TcpPacket, stream: &mut TcpStream) {
    let cmd = TcpPacket::command(TcpPacketCommand::Save).to_bytes();
    let buf: &[u8] = &cmd.as_slice();

    let res = stream.write_all(buf);

    match res {
        Ok(_) => {}
        Err(_) => warn("data not sent succesfully"),
    }
}

fn handle_get(_: TcpPacket, stream: &mut TcpStream) {
    let cmd = TcpPacket::command(TcpPacketCommand::Get)
        .to_bytes();

    let buf: &[u8] = &cmd.as_slice();

    let res = stream.write_all(buf);

    match res {
        Ok(_) => {}
        Err(_) => warn("data not sent succesfully"),
    }
}
