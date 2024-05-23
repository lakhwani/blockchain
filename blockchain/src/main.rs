mod blockchain;
mod cli;
mod client;
mod server;
mod transaction;

use cli::help_text;
use client::send_command;
use std::io::stdin;
fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0); // remove the program name
    println!("{}", help_text());

    println!("Enter command:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input.starts_with("b ") {
        let command = input[2..].to_string();
        if command == "start-node" {
            server::start_server();
        } else {
            send_command(command);
            client::run_client_loop();
        }
    } else {
        println!("Commands should start with 'b '.");
    }
    return;
}
