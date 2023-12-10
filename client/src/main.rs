use std::{net::TcpStream, thread};
use std::sync::mpsc::channel;

use better_term::Color;
use aether_common::{say, nay, hey, yay};
use send_it::{writer::VarWriter, reader::VarReader};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Config};

// reads changes from the server and implements them locally
fn server_reader(mut stream: TcpStream) {
    let mut reader = VarReader::new(&mut stream);

    while let Ok(read) = reader.read_data() {
        // todo: this is where you should handle incoming data from the server
    }
}

fn handle_file_changes(){
    say!("Handling file change...");
}


// watches files and sends changes to the server
fn watcher(stream: TcpStream) {
    let mut writer = VarWriter::default();
    let (tx, rx) = channel();

    // todo(ben): this is where you should detect and write changes to the server

    // current exe directory
    // LATER EXE WILL BE IN .obsidian FOLDER
    // MAKE PATH ESCAPE .obsidian
    let exe_dir = std::env::current_exe().unwrap();
    let vault_dir = exe_dir
        .parent()
        .and_then(|parent| parent.parent())
        .and_then(|parent| parent.parent())
        .map(|parent| parent.join("test-vault"))
        .expect("Failed to construct test vault path");
    say!("Directory: {}", &vault_dir.to_string_lossy());

    // forever loop watching for changes
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();

    watcher.watch(&vault_dir, RecursiveMode::Recursive).unwrap();

    say!("Watcher has begun...");

    loop {
        match rx.recv() {
            Ok(event) => {
                let Ok(e) = event else {
                    nay!("Failed to get event from receiver");
                    return;
                };
                match e.kind {
                    notify::EventKind::Any => hey!("Somehow its 'Any'"),
                    notify::EventKind::Access(access) => yay!("Accessed file: {:?}", access),
                    notify::EventKind::Create(create) => yay!("Created file: {:?}", create),
                    notify::EventKind::Modify(modify) => say!("Modified file: {:?}", modify),
                    notify::EventKind::Remove(remove) => nay!("Removed file: {:?}", remove),
                    notify::EventKind::Other => hey!("Somehow its 'Other'"),
                }
            }
            Err(e) => nay!("Watcher error: {:?}", e),
        }
    }
}

fn main() {
    // todo: add a way for the user to connect to a server. I reccomend using a toml file for configuration!
    let ip = "localhost";
    let port = "3333";

    // connect to the server
    say!("Attempting connection to {}:{}", ip, port);
    let Ok(connection) = TcpStream::connect(format!("{}:{}", ip, port)) else {
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
