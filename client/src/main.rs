use std::net::TcpStream;

use logger::{say, nay, hey};
use send_it::writer::VarWriter;

fn main() {
    let ip = "localhost";
    let port = "3333";
    say!("Attempting connection to {}:{}", ip, port);
    let Ok(mut connection) = TcpStream::connect(format!("{}:{}", ip, port)) else {
        nay!("Failed to connect to server! Check the ip and port are correct!");
        return;
    };

    let mut writer = VarWriter::default();

    writer.add_string("This is a test!");

    writer.send(&mut connection).expect("Failed to send data to the server!");

    hey!("Connection to the server has been terminated!");
}
