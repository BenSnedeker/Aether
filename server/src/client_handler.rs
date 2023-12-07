use std::net::TcpStream;

use better_term::Color;
use logger::{yay, hey};
use send_it::{reader::VarReader, writer::VarWriter};

pub fn handle_client(mut stream: &mut TcpStream, ip: String) {
    yay!("Accepted incoming connection from {}{}{}.", Color::White, ip, Color::BrightGreen);

    // create the reader and writer
    let mut reader = VarReader::new(&mut stream);
    
    // todo: add a separate handler for writing to the client if needed
    let mut writer = VarWriter::default();

    // read loop from client
    while let Ok(data) = reader.read_data() {
        // todo: process data
    }
    hey!("Client with ip {}{}{} has disconnected.", Color::White, ip, Color::BrightYellow);
}