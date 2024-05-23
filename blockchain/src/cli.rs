pub enum Command {
    CreateAccount(u32, u32),
    Transfer(u32, u32, u32),
    Balance(u32),
    Help,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0] {
        "create-account" => {
            if parts.len() != 3 {
                return Err("Usage: create-account <id-of-account> <starting-balance>".to_string());
            }
            let id = parts[1].parse::<u32>().map_err(|_| "Invalid account ID")?;
            let balance = parts[2]
                .parse::<u32>()
                .map_err(|_| "Invalid starting balance")?;
            Ok(Command::CreateAccount(id, balance))
        }
        "transfer" => {
            if parts.len() != 4 {
                return Err("Usage: transfer <from-account> <to-account> <amount>".to_string());
            }
            let from = parts[1]
                .parse::<u32>()
                .map_err(|_| "Invalid from account ID")?;
            let to = parts[2]
                .parse::<u32>()
                .map_err(|_| "Invalid to account ID")?;
            let amount = parts[3].parse::<u32>().map_err(|_| "Invalid amount")?;
            Ok(Command::Transfer(from, to, amount))
        }
        "balance" => {
            if parts.len() != 2 {
                return Err("Usage: balance <account>".to_string());
            }
            let id = parts[1].parse::<u32>().map_err(|_| "Invalid account ID")?;
            Ok(Command::Balance(id))
        }
        "help" => Ok(Command::Help),
        _ => Err("Unknown command. Type 'help' for a list of commands.".to_string()),
    }
}

pub fn help_text() -> String {
    let help = "Available commands:\n\
                b start-node                                        - Start the blockchain node\n\
                b create-account <id-of-account> <starting-balance> - Create a new account with a starting balance\n\
                b transfer <from-account> <to-account> <amount>     - Transfer funds from one account to another\n\
                b balance <account>                                 - Check the balance of an account\n\
                b help                                              - Show this help message\n";
    help.to_string()
}
