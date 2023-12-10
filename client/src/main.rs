use std::{net::TcpStream, thread};

use better_term::Color;
use aether_common::{say, nay, hey, yay};
use send_it::{writer::VarWriter, reader::VarReader};

// reads changes from the server and implements them locally
fn server_reader(mut stream: TcpStream) {
    let mut reader = VarReader::new(&mut stream);

    while let Ok(read) = reader.read_data() {
        // todo: this is where you should handle incoming data from the server
    }
}

// watches files and sends changes to the server
fn watcher(mut stream: TcpStream) {
    let mut writer = VarWriter::default();

    // todo: this is where you should detect and write changes to the server
}

fn main() {
    // todo: add a way for the user to connect to a server. I reccomend using a toml file for configuration!
    let ip = "localhost";
    let port = "3333";

    // connect to the server
    say!("Attempting connection to {}:{}", ip, port);
    let Ok(mut connection) = TcpStream::connect(format!("{}:{}", ip, port)) else {
        nay!("Failed to connect to server! Check the ip and port are correct!");
        return;
    };
    // log the connection
    yay!("Connected to {}{}:{}{} successfully!", Color::White, ip, port, Color::BrightGreen);

    // spawn the server reading thread
    let Ok(stream_cpy) = connection.try_clone() else {
        nay!("Failed to copy stream to send to thread!");
        return;
    };
    thread::spawn(move|| server_reader(stream_cpy));

    // start the watcher
    watcher(connection);

    hey!("Connection to the server has been terminated!");
}
