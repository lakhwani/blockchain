use std::io::{stdin, Read, Write};
use std::net::TcpStream;

pub fn send_command(command: String) {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            stream.write(command.as_bytes()).unwrap();
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    println!("{}", String::from_utf8_lossy(&buffer[..size]));
                }
                Err(e) => {
                    println!("Failed to read response: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

pub fn run_client_loop() {
    loop {
        println!("Enter command:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.starts_with("b ") {
            let command = input[2..].to_string();
            send_command(command);
        } else {
            println!("Commands should start with 'b '.");
        }
    }
}
