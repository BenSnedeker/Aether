use std::{net::TcpStream, thread};

use better_term::Color;
use logger::{yay, hey, say, nay};
use send_it::{reader::VarReader, writer::VarWriter};

use crate::buffer::SuperBuff;

// reads from the client
pub fn client_reader(mut stream: TcpStream, buffer: SuperBuff<String>, ip: String) {
    let mut reader = VarReader::new(&mut stream);
    
    // read loop from client
    while let Ok(data) = reader.read_data() {
        for seg in data {
            say!("Segment from client {}{}{}: {}", Color::White, ip, Color::BrightBlack, seg.to_string());
        }
    }
}

// reads from local files and sends to the clients
pub fn client_writer(stream: TcpStream, ip: String) {
    let mut writer = VarWriter::default();

    // todo: send data to the client if a file changes or something, idk
}

pub fn handle_client(stream: TcpStream, buffer: SuperBuff<String>, ip: String) {
    yay!("Accepted incoming connection from {}{}{}.", Color::White, ip, Color::BrightGreen);

    let Ok(stream_copy) = stream.try_clone() else {
        nay!("Failed to copy stream to send to thread!");
        return;
    };

    let ip_clone = ip.clone();

    thread::spawn(move|| client_reader(stream_copy, buffer, ip_clone));

    // handle the writer here
    client_writer(stream, ip.clone());

    hey!("Client with ip {}{}{} has disconnected.", Color::White, ip, Color::BrightYellow);
}