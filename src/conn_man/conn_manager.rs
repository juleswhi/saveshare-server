use crate::state::packet::*;

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
    let mut cmd = TcpPacket::command(TcpPacketCommand::Save);

    let save_result = db::get_saves();

    if save_result.is_none() {
        cmd.add_failure();
    }

    let bytes = cmd.to_bytes();
    let buf: &[u8] = &bytes.as_slice();

    let res = stream.write_all(buf);

    match res {
        Ok(_) => {}
        Err(_) => warn("data not sent succesfully"),
    }
}

fn handle_get(_: TcpPacket, stream: &mut TcpStream) {
    let mut cmd = TcpPacket::command(TcpPacketCommand::Get);

    let get_result = db::save();

    if get_result.is_none() {
        cmd.add_failure();
    }

    let bytes = cmd.to_bytes();
    let buf: &[u8] = &bytes.as_slice();

    let res = stream.write_all(buf);

    match res {
        Ok(_) => {}
        Err(_) => warn("data not sent succesfully"),
    }
}
