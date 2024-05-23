use std::io::{stdin, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod transaction;
use transaction::{CreateAccount, TransactionType, Transfer};

mod blockchain;
use blockchain::Blockchain;

mod cli;
use cli::{help_text, parse_command, Command};

fn handle_client(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0; 512];
    let mut stream = stream;

    match stream.read(&mut buffer) {
        Ok(size) => {
            let input = str::from_utf8(&buffer[..size]).expect("Failed to parse input");
            match parse_command(input.trim()) {
                Ok(Command::CreateAccount(id, balance)) => {
                    let tx = TransactionType::CreateAccountTx(CreateAccount {
                        account_id: id,
                        starting_balance: balance,
                    });
                    let mut blockchain = blockchain.lock().unwrap();
                    blockchain.add_transaction(tx);
                    stream
                        .write(b"Create account transaction added.\n")
                        .unwrap();
                }
                Ok(Command::Transfer(from, to, amount)) => {
                    let tx = TransactionType::TransferTx(Transfer {
                        from_account: from,
                        to_account: to,
                        amount,
                    });
                    let mut blockchain = blockchain.lock().unwrap();
                    blockchain.add_transaction(tx);
                    stream.write(b"Transfer transaction added.\n").unwrap();
                }
                Ok(Command::Balance(id)) => {
                    let blockchain = blockchain.lock().unwrap();
                    match blockchain.accounts.get(&id) {
                        Some(balance) => {
                            let response = format!("Balance of account {}: {}\n", id, balance);
                            stream.write(response.as_bytes()).unwrap();
                        }
                        None => {
                            let response = format!("Account {} not found.\n", id);
                            stream.write(response.as_bytes()).unwrap();
                        }
                    }
                }
                Ok(Command::Help) => {
                    let help_text = help_text();
                    stream.write(help_text.as_bytes()).unwrap();
                }
                Err(e) => {
                    let error_message = format!("{}\n", e);
                    stream.write(error_message.as_bytes()).unwrap();
                }
            }
        }
        Err(_) => {
            println!("Failed to read from connection");
        }
    }
}

fn start_server() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    println!("Starting B blockchain server...");

    let blockchain_clone = Arc::clone(&blockchain);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let mut blockchain = blockchain_clone.lock().unwrap();
        blockchain.mine_block();
        println!(
            "New block mined at T={}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    });

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let blockchain_clone = Arc::clone(&blockchain);
                thread::spawn(move || {
                    handle_client(stream, blockchain_clone);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn send_command(command: String) {
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

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0); // remove the program name

    if args.is_empty() {
        println!("Usage: b <command>");
        println!("{}", help_text());
        return;
    }

    if args[0] == "b" {
        args.remove(0); // remove the "b" command prefix
    } else {
        println!("Commands should start with 'b'.");
        return;
    }

    if args.is_empty() {
        println!("Usage: b <command>");
        println!("{}", help_text());
        return;
    }

    let command = args.join(" ");
    if command == "start-node" {
        start_server();
    } else {
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
}
