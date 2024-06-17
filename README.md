# B Blockchain

This is a prototype blockchain implementation in Rust. It supports creating accounts, transferring funds between accounts, and checking account balances. Blocks are mined at regular intervals.

## Project Structure

```

blockchain/
│
├── src/
│ ├── main.rs
│ ├── client.rs
│ ├── server.rs
│ ├── transaction.rs
│ ├── blockchain.rs
│ └── cli.rs
│
└── Cargo.toml

```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) must be installed on your machine.

### Running the Application

1. **Start the Blockchain Server:**

   Open a terminal and run the following command to start the blockchain server:

   ```sh
   cargo run
   ```

Then type the following command to start the server:

```
b start-node
```

The server will start running and mining blocks every 10 seconds.

2. **Interacting with the Server:**

   Open another terminal window and navigate to the project directory. You can then run the following commands to interact with the blockchain server:

   **Create an Account:**

   ```sh
   cargo run
   ```

   Then type the following command to create an account with ID `1` and starting balance `1000`:

   ```
   b create-account 1 1000
   ```

   **Transfer Funds:**

   ```sh
   cargo run
   ```

   Then type the following command to transfer `100` units from account `1` to account `2`:

   ```
   b create-account 2 10
   b transfer 1 2 100
   ```

   **Check Account Balance:**

   ```
   b balance 1
   b balance 2
   ```

   **Help:**

   ```sh
   cargo run
   ```

   Then type the following command to display the help message:

   ```
   b help
   ```

## Commands

- `b start-node`: Start the blockchain node. This should be run in a separate terminal.
- `b create-account <id> <starting-balance>`: Create a new account with the specified ID and starting balance.
- `b transfer <from-account> <to-account> <amount>`: Transfer funds from one account to another.
- `b balance <account>`: Check the balance of the specified account.
- `b help`: Show the help message with available commands.
