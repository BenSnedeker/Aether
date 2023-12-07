use std::{net::TcpListener, thread};

use logger::{hey, nay, say, yay};

use crate::client_handler::handle_client;

mod client_handler;

fn main() {
    let ip = "0.0.0.0";
    let port = "3333";
    say!("Starting client on {}:{}", ip, port);

    let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
        Ok(l) => l,
        Err(e) => {
            nay!("Failed to bind to address! Maybe try a different port?");
            return;
        }
    };
    yay!("Listening on port {}", port);

    // handle incoming connections
    for stream in listener.incoming() {
        if let Err(e) = stream {
            hey!("Error accepting incoming connection: {}", e);
            continue;
        }

        let mut stream = stream.unwrap();

        thread::spawn(move|| {
            // get the ip of the client
            let ip_result  = stream.peer_addr();
            if let Err(e) = ip_result {
                hey!("Failed to get a client's IP address! Error: {}", e);
                return;
            }
            let ip = ip_result.unwrap();
            handle_client(&mut stream, ip.to_string());
        });
    }
    hey!("Closing the server!");
}
