dfx start --background
dfx canister create token_wallet
dfx deploy
dfx canister call token_wallet get_balance
dfx canister call token_wallet send_tokens '(principal "<other_wallet_principal>", 10)'
use ic_cdk::api::call;
use ic_cdk::export::Principal;
use ic_cdk::storage;
use super::TokenWallet;

#[test]
fn test_send_tokens() {
    let mut wallet = TokenWallet::default();
    let user_1 = Principal::from_text("user_1").unwrap();
    let user_2 = Principal::from_text("user_2").unwrap();

    wallet.receive_tokens(100);
    wallet.send_tokens(user_2, 50).unwrap();

    assert_eq!(wallet.get_balance(), 50); // User 1 should have 50 left
    assert_eq!(wallet.balances.get(&user_2).unwrap(), &50); // User 2 should have 50
}

#[test]
fn test_insufficient_balance() {
    let mut wallet = TokenWallet::default();
    let user_1 = Principal::from_text("user_1").unwrap();

    wallet.receive_tokens(50);

    assert!(wallet.send_tokens(user_1, 100).is_err()); // Should fail due to insufficient balance
}
