# Quadb_Blockchain_Assignment
It is a Internship Assignment
1. Setting Up Your Environment:
To get started with ICP development using Rust, you'll need to install a few tools:

Install Rust: You need Rust installed on your system. Follow the official guide here.
Install dfx: The ICP framework tool dfx is required to develop, deploy, and interact with the local ICP testnet. Install it by following the guide on IC's official documentation.
Create a New Project: You can create a new project by running the following command in your terminal:

This creates a basic setup for your project.

2. Smart Contract Development (Rust)
In the ICP blockchain, smart contracts are deployed as canisters. You’ll develop a canister that handles token transactions (IRCRC2), allowing you to send, receive, and display token balances.

Create a Smart Contract:

Create a token_wallet canister that implements the basic functionalities. The following steps cover each aspect.

a. Define Smart Contract Logic (Rust):
In the src/token_wallet/main.rs file, you can define your smart contract as follows:

This is a simple implementation that includes:

Token sending (send_tokens): Sends tokens from one address to another.
Token receiving (receive_tokens): A simple implementation where tokens can be added to the sender’s balance.
Balance fetching (get_balance): Fetches the balance of the caller.
b. Deploy the Canister:
Run the following commands to deploy your canister to the ICP test network:

c. Interacting with the Wallet:
You can test your token wallet by calling the exposed functions via the command line.
Example:
To send tokens:
3. Unit Testing
Now that you have the basic contract, you need to write unit tests using the ICP framework.

In tests/test.rs, write tests for the send_tokens, receive_tokens, and get_balance functions.

Run tests with:

4. Security Considerations
To ensure security:

Ensure private keys are not stored in the smart contract.
Use cryptographic signatures to verify requests.
Secure the deployment process and restrict access to sensitive actions (e.g., token transfers).
5. Documentation and Setup Instructions
In the README.md, include:

Prerequisites:
Rust and dfx installed
ICP Test Network access
Setup:
Clone the repo and navigate to the project directory.
Run dfx start to initialize the ICP test environment.
Deploy with dfx deploy.
Usage:
Call send_tokens, receive_tokens, and get_balance via dfx canister call.
6. GitHub Submission
Once you have completed the development, testing, and documentation, push your code to a GitHub repository and share the link as your submission.
