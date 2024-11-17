use ic_cdk::api::{call, data, print};
use ic_cdk::export::Principal;
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(Default)]
pub struct TokenWallet {
    // Mapping from user address to token balance
    balances: HashMap<Principal, u64>,
}

impl TokenWallet {
    // Send tokens to another address
    pub fn send_tokens(&mut self, to: Principal, amount: u64) -> Result<(), String> {
        let caller = ic_cdk::caller();
        let from_balance = self.balances.entry(caller.clone()).or_insert(0);

        // Check if sender has enough balance
        if *from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *from_balance -= amount;
        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;

        Ok(())
    }

    // Receive tokens (simplified here)
    pub fn receive_tokens(&mut self, amount: u64) {
        let caller = ic_cdk::caller();
        let balance = self.balances.entry(caller).or_insert(0);
        *balance += amount;
    }

    // Fetch the current token balance of the wallet
    pub fn get_balance(&self) -> u64 {
        let caller = ic_cdk::caller();
        *self.balances.get(&caller).unwrap_or(&0)
    }
}

// Expose methods to the outside world
#[ic_cdk_macros::query]
fn get_balance() -> u64 {
    let wallet: TokenWallet = ic_cdk::storage::get();
    wallet.get_balance()
}

#[ic_cdk_macros::update]
fn send_tokens(to: Principal, amount: u64) -> Result<(), String> {
    let mut wallet: TokenWallet = ic_cdk::storage::get();
    wallet.send_tokens(to, amount)?;
    ic_cdk::storage::set(wallet);
    Ok(())
}

#[ic_cdk_macros::update]
fn receive_tokens(amount: u64) {
    let mut wallet: TokenWallet = ic_cdk::storage::get();
    wallet.receive_tokens(amount);
    ic_cdk::storage::set(wallet);
}
