use std::io::{Read, Write};
use std::net::TcpListener;

use crate::commands::handle_command;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer);
                let input = String::from_utf8_lossy(&buffer);

                let response = handle_command(input.trim());

                let _ = stream.write(response.as_bytes());
            }

            Err(e) => println!("connection falied: {}", e),
        }
    }
}
