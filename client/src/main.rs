use std::net::TcpStream;

use better_term::Color;
use logger::{say, nay, hey, yay};
use send_it::writer::VarWriter;

fn main() {
    let ip = "localhost";
    let port = "3333";
    say!("Attempting connection to {}:{}", ip, port);
    let Ok(mut connection) = TcpStream::connect(format!("{}:{}", ip, port)) else {
        nay!("Failed to connect to server! Check the ip and port are correct!");
        return;
    };
    
    yay!("Connected to {}{}:{}{} successfully!", Color::White, ip, port, Color::BrightGreen);

    let mut writer = VarWriter::default();

    writer.add_string("This is a test!");

    writer.send(&mut connection).expect("Failed to send data to the server!");

    hey!("Connection to the server has been terminated!");
}
