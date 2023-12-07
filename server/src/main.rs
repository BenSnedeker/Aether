use std::{net::TcpListener, thread};

use better_term::Color;
use logger::{hey, nay, say, yay};

use crate::{client_handler::handle_client, buffer::SuperBuff};

mod client_handler;
mod buffer;

fn main() {
    let ip = "0.0.0.0";
    let port = "3333";
    say!("Starting server on {}{}:{}", Color::White, ip, port);

    let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
        Ok(l) => l,
        Err(e) => {
            nay!("Failed to bind to address! Error: {}", e);
            return;
        }
    };
    yay!("Listening on port {}", port);

    // create the change buffer
    let buffer: SuperBuff<String> = SuperBuff::new();

    // handle incoming connections
    for stream in listener.incoming() {
        if let Err(e) = stream {
            hey!("Error accepting incoming connection: {}", e);
            continue;
        }

        let stream = stream.unwrap();

        let buffer_clone = buffer.clone();

        thread::spawn(move|| {
            // get the ip of the client
            let ip_result  = stream.peer_addr();
            if let Err(e) = ip_result {
                hey!("Failed to get a client's IP address! Error: {}", e);
                return;
            }
            let ip = ip_result.unwrap();
            handle_client(stream, buffer_clone, ip.to_string());
        });
    }
    hey!("Closing the server!");
}
