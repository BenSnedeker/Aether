use std::{net::TcpListener, thread};

use better_term::Color;
use aether_common::{hey, nay, say, yay, change::ChangeType};

use crate::{client_handler::handle_client, buffer::SuperBuff};

mod client_handler;
mod buffer;
mod log;

fn buffer_handler(mut buffer: SuperBuff<ChangeType>) {
    // loop until the program stops
    loop {
        // get the next item in the buffer
        if !buffer.has_next() { continue; }
        let Some(next) = buffer.pop() else { continue; };

        // todo(ben): handle the `next` change in the buffer
        
    }
}

fn main() {
    // todo(ben): these could be configurable
    let ip = "0.0.0.0";
    let port = "3333";

    // print out starting information
    say!("Starting server on {}{}:{}", Color::White, ip, port);

    // create the TcpListener
    let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
        Ok(l) => l,
        Err(e) => {
            nay!("Failed to bind to address! Error: {}", e);
            return;
        }
    };
    yay!("Listening on port {}", port);

    // create the change buffer
    let buffer: SuperBuff<ChangeType> = SuperBuff::new();

    // handle the buffer
    let buffer_copy = buffer.clone();
    thread::spawn(move|| buffer_handler(buffer_copy));

    // handle incoming connections
    for stream in listener.incoming() {
        // handle the case where the stream connection fails
        if let Err(e) = stream {
            hey!("Error accepting incoming connection: {}", e);
            continue;
        }

        // get the stream into a varaible
        let stream = stream.unwrap();

        // clone the buffer to send to the thread
        let buffer_clone = buffer.clone();

        // spawn a new thread for the client
        // this would not be optimal for servers expecting a high number of clients, for that use an asyncronous crate like async-std or tokio
        thread::spawn(move|| {
            // get the ip of the client
            let ip_result  = stream.peer_addr();
            if let Err(e) = ip_result {
                hey!("Failed to get a client's IP address! Error: {}", e);
                return;
            }
            let ip = ip_result.unwrap();

            // start the client handler with the required parameters
            handle_client(stream, buffer_clone, ip.to_string());
        });
    }

    hey!("Closing the server!");
}
