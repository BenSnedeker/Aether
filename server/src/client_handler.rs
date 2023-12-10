use std::{net::TcpStream, thread};

use better_term::Color;
use aether_common::{yay, hey, say, nay, change::Change};
use send_it::{reader::VarReader, writer::VarWriter};

use crate::buffer::SuperBuff;

// reads from the client
pub fn client_reader(mut stream: TcpStream, buffer: SuperBuff<Change>, ip: String) {
    // create the reader wrapping the TcpStream
    let mut reader = VarReader::new(&mut stream);
    
    // read loop from client
    while let Ok(data) = reader.read_data() {
        // TODO: This is where incoming changes from the client will be recieved.

        // loop through all segments (I reccomend you have a fixed segment structure)
        for seg in data {
            say!("Segment from client {}{}{}: {}", Color::White, ip, Color::BrightBlack, seg.to_string());
        }
    }
}

// reads from local files and sends to the clients
pub fn client_writer(stream: TcpStream, ip: String) {
    // create the writer
    let mut writer = VarWriter::default();

    // TODO: send data to the client about local file changes
}

pub fn handle_client(stream: TcpStream, buffer: SuperBuff<Change>, ip: String) {
    // print successful connection
    yay!("Accepted incoming connection from {}{}{}.", Color::White, ip, Color::BrightGreen);

    // clone the stream for the reader thread
    let Ok(stream_copy) = stream.try_clone() else {
        nay!("Failed to copy stream to send to thread!");
        return;
    };

    // clone the ip for the reader thread
    let ip_clone = ip.clone();

    // spawn the reader / listener thread to read data from the client
    thread::spawn(move|| client_reader(stream_copy, buffer, ip_clone));

    // handle writing changes to the client
    client_writer(stream, ip.clone());

    hey!("Client with ip {}{}{} has disconnected.", Color::White, ip, Color::BrightYellow);
}