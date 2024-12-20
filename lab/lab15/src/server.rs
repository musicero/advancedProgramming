use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::constants;
use crate::game::Game;
use crate::game_cmd::*;

pub fn server() {
    println!("Server started, Waiting for connections");
    loop {
        // binding server to port 8080
        let Ok(listener) = TcpListener::bind(constants::SERVER_ADDRESS) else {
            println!("Error binding server to port 8080. Retrying...");
            thread::sleep(Duration::from_secs(1));
            continue;
        };

        let mut pending: Option<TcpStream> = None;

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                match pending.take() {
                    Some(mut pending_stream) => {
                        println!("Two players connected");
                        handle_stream_pair(&mut stream, &mut pending_stream);
                    }
                    None => {
                        println!("One player connected");
                        stream
                            .write(b"Waiting for opponent")
                            .expect("Error writing to stream");
                        pending = Some(stream);
                    }
                }
            }
        }
    }
}

fn handle_stream_pair(stream1: &mut TcpStream, stream2: &mut TcpStream) {
    stream1
        .set_read_timeout(Some(Duration::from_millis(200)))
        .expect("Error setting read timeout");
    stream2
        .set_read_timeout(Some(Duration::from_millis(200)))
        .expect("Error setting read timeout");

    let mut game = Game::new((19, 19));
    send(stream1, "Game started");
    send(stream2, "Game started");
    loop {
        handle_stream(&mut game, stream1, stream2);
        handle_stream(&mut game, stream2, stream1);
    }
}

fn handle_stream(game: &mut Game, this_stream: &mut TcpStream, other_stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let reply = match this_stream.read(&mut buffer) {
        Ok(0) => Some(Reply {
            this: None,
            other: Some("Opponent disconnected\n".to_string()),
        }),
        Ok(bytes_read) => {
            let command = String::from_utf8_lossy(&buffer[..bytes_read]);
            Some(handle_command(game, command.trim().to_string()))
        }
        // Nothing was sent
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => None,
        Err(e) => panic!("Error reading from stream: {:?}", e),
    };

    // send replies to clients
    match reply {
        Some(reply) => {
            if let Some(this) = reply.this {
                send(this_stream, this.as_str());
            }
            if let Some(other) = reply.other {
                send(other_stream, other.as_str());
            }
        }
        None => (),
    }
}

fn send(stream: &mut TcpStream, message: &str) {
    stream
        .write(message.as_bytes())
        .expect("Error writing to stream");
}
