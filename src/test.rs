#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
};

use crate::TokenContract;
use crate::TokenContractClient;

#[test]
fn test_initialize() {

    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin, &user, &1000);

    let balance = client.balance(&user);

    assert_eq!(balance, 1000);
}

#[test]
fn test_mint() {

    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin, &user, &1000);

    client.mint(&user, &500);

    let balance = client.balance(&user);

    assert_eq!(balance, 1500);
}

#[test]
fn test_transfer() {

    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let user1 = Address::generate(&env);

    let user2 = Address::generate(&env);

    client.initialize(&admin, &user1, &1000);

    client.transfer(&user1, &user2, &300);

    assert_eq!(client.balance(&user1), 700);

    assert_eq!(client.balance(&user2), 300);
}

#[test]
fn test_burn() {

    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let user = Address::generate(&env);

    client.initialize(&admin, &user, &1000);

    client.burn(&user, &200);

    assert_eq!(client.balance(&user), 800);
}

#[test]
fn test_approve_and_transfer_from() {

    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let owner = Address::generate(&env);

    let spender = Address::generate(&env);

    let receiver = Address::generate(&env);

    client.initialize(&admin, &owner, &1000);

    client.approve(&owner, &spender, &500,&1000);

    client.transfer_from(
        &spender,
        &owner,
        &receiver,
        &300,
    );

    assert_eq!(client.balance(&owner), 700);

    assert_eq!(client.balance(&receiver), 300);

    assert_eq!(
        client.allowance(&owner, &spender),
        200
    );
}

#[test]
fn test_burn_from() {

    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let owner = Address::generate(&env);

    let spender = Address::generate(&env);

    client.initialize(&admin, &owner, &1000);

    client.approve(&owner, &spender, &400, &1000);

    client.burn_from(
        &spender,
        &owner,
        &200,
    );

    assert_eq!(client.balance(&owner), 800);

    assert_eq!(
        client.allowance(&owner, &spender),
        200
    );
}