use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::constants;

pub fn client() {
    println!("Connecting to the server...");
    loop {
        match TcpStream::connect(constants::SERVER_ADDRESS) {
            Ok(stream) => {
                user_session(stream);
                break;
            }
            Err(_) => {
                println!("Failed to connect to the server. Retrying...");
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn user_session(mut stream: TcpStream) {
    stream
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .expect("Error setting read timeout");

    msg_user("Connected to the server");

    loop {
        listen_server(&mut stream);
        listen_user(&mut stream);
    }
}

fn listen_server(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(0) => {
            println!("Connection to server is closed");
            std::process::exit(0);
        }
        Ok(bytes_read) => {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            if response != "OK" && !response.trim().is_empty() {
                msg_user(format!("SERVER: {}", response).as_str());
            }
        }
        Err(e) => {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                return;
            }
        }
    }
}

fn listen_user(stream: &mut TcpStream) {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    let input = input.trim();
    // send input to server
    match stream.write(input.as_bytes()) {
        Ok(_) => {
            if input == "exit" {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
        Err(e) => {
            println!("Failed to send data to server: {}", e);
        }
    }
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(bytes) => {
            let response = String::from_utf8_lossy(&buffer[..bytes]);
            msg_user(format!("SERVER: {}", response).as_str());
        }

        Err(e) => {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                println!("Failed to read from server: {}", e);
                return;
            }
        }
    }
}

fn msg_user(msg: &str) {
    println!("\n{}", msg);
    print!("Enter command: ");
    std::io::stdout().flush().unwrap();
}
