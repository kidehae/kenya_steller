#![no_std]

mod storage;

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract,
    contractimpl,
    token::Interface,
    Address,
    Env,
    MuxedAddress,
    String,
};

use storage::DataKey;

#[contract]
pub struct TokenContract;

fn read_balance(env: &Env, user: Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(user))
        .unwrap_or(0)
}

fn write_balance(env: &Env, user: Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(user), &amount);
}

#[contractimpl]
impl TokenContract {

    pub fn initialize(
        env: Env,
        admin: Address,
        to: Address,
        amount: i128,
    ) {

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);

        write_balance(&env, to, amount);
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "MyToken")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "MTK")
    }

    pub fn decimals(_env: Env) -> u32 {
        7
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        read_balance(&env, id)
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) {

        from.require_auth();

        let from_balance = read_balance(&env, from.clone());

        assert!(from_balance >= amount, "insufficient balance");

        let to_balance = read_balance(&env, to.clone());

        write_balance(&env, from, from_balance - amount);

        write_balance(&env, to, to_balance + amount);
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        _expiration_ledger: u32,
    ) {

        from.require_auth();

        env.storage()
            .persistent()
            .set(
                &DataKey::Allowance(from, spender),
                &amount,
            );
    }

    pub fn allowance(
        env: Env,
        from: Address,
        spender: Address,
    ) -> i128 {

        env.storage()
            .persistent()
            .get(&DataKey::Allowance(from, spender))
            .unwrap_or(0)
    }

    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
    ) {

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        let balance = read_balance(&env, to.clone());

        write_balance(&env, to, balance + amount);
    }

    pub fn burn(
        env: Env,
        from: Address,
        amount: i128,
    ) {

        from.require_auth();

        let balance = read_balance(&env, from.clone());

        assert!(balance >= amount, "insufficient balance");

        write_balance(&env, from, balance - amount);
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {

        spender.require_auth();

        let allowance_key =
            DataKey::Allowance(from.clone(), spender.clone());

        let allowance: i128 = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap_or(0);

        assert!(allowance >= amount, "allowance exceeded");

        let from_balance = read_balance(&env, from.clone());

        assert!(from_balance >= amount, "insufficient balance");

        let to_balance = read_balance(&env, to.clone());

        write_balance(&env, from, from_balance - amount);

        write_balance(&env, to, to_balance + amount);

        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));
    }

    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) {

        spender.require_auth();

        let allowance_key =
            DataKey::Allowance(from.clone(), spender.clone());

        let allowance: i128 = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap_or(0);

        assert!(allowance >= amount, "allowance exceeded");

        let balance = read_balance(&env, from.clone());

        assert!(balance >= amount, "insufficient balance");

        write_balance(&env, from, balance - amount);

        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));
    }
}

impl Interface for TokenContract {

    fn allowance(
        env: Env,
        from: Address,
        spender: Address,
    ) -> i128 {
        Self::allowance(env, from, spender)
    }

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        Self::approve(
            env,
            from,
            spender,
            amount,
            expiration_ledger,
        )
    }

    fn balance(
        env: Env,
        id: Address,
    ) -> i128 {
        Self::balance(env, id)
    }

    fn transfer(
        env: Env,
        from: Address,
        to: MuxedAddress,
        amount: i128,
    ) {

        let to_address = to.address();

        Self::transfer(
            env,
            from,
            to_address,
            amount,
        )
    }

    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        Self::transfer_from(
            env,
            spender,
            from,
            to,
            amount,
        )
    }

    fn burn(
        env: Env,
        from: Address,
        amount: i128,
    ) {
        Self::burn(env, from, amount)
    }

    fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) {
        Self::burn_from(
            env,
            spender,
            from,
            amount,
        )
    }

    fn decimals(env: Env) -> u32 {
        Self::decimals(env)
    }

    fn name(env: Env) -> String {
        Self::name(env)
    }

    fn symbol(env: Env) -> String {
        Self::symbol(env)
    }
}