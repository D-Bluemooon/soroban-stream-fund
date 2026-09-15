//! soroban-stream-fund
//!
//! A minimal per-second token streaming contract for Soroban.
//!
//! This lets a `sender` deposit tokens once and have them become
//! withdrawable by a `recipient` continuously over time, at a fixed
//! `rate_per_second`, instead of as a single lump-sum transfer.
//!
//! This is intentionally small in scope: no cancellation, no top-ups,
//! no multi-recipient splitting. Those are good candidates for the
//! open issues in this repo (see ISSUES.md) and are left out on
//! purpose so the contract stays easy to read and audit.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct Stream {
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub rate_per_second: i128,
    pub last_withdrawn: u64,
    pub balance: i128,
}

#[contracttype]
pub enum DataKey {
    Stream(u64),
    NextId,
}

#[contract]
pub struct StreamFundContract;

#[contractimpl]
impl StreamFundContract {
    /// Create a new stream. Pulls `deposit` from `sender` into the
    /// contract and returns the new stream's id.
    pub fn create_stream(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        deposit: i128,
        rate_per_second: i128,
    ) -> u64 {
        sender.require_auth();
        assert!(deposit > 0, "deposit must be positive");
        assert!(rate_per_second > 0, "rate must be positive");

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&sender, &env.current_contract_address(), &deposit);

        let id = Self::next_id(&env);
        let stream = Stream {
            sender,
            recipient,
            token,
            rate_per_second,
            last_withdrawn: env.ledger().timestamp(),
            balance: deposit,
        };
        env.storage().persistent().set(&DataKey::Stream(id), &stream);
        id
    }

    /// Withdraw everything currently owed to the recipient. Returns the
    /// amount actually transferred.
    pub fn withdraw(env: Env, id: u64) -> i128 {
        let mut stream: Stream = env
            .storage()
            .persistent()
            .get(&DataKey::Stream(id))
            .expect("stream not found");
        stream.recipient.require_auth();

        let now = env.ledger().timestamp();
        let elapsed = now.saturating_sub(stream.last_withdrawn);
        let mut amount = (elapsed as i128).saturating_mul(stream.rate_per_second);
        if amount > stream.balance {
            amount = stream.balance;
        }

        if amount > 0 {
            let token_client = token::Client::new(&env, &stream.token);
            token_client.transfer(&env.current_contract_address(), &stream.recipient, &amount);
            stream.balance -= amount;
        }
        stream.last_withdrawn = now;
        env.storage().persistent().set(&DataKey::Stream(id), &stream);

        amount
    }

    /// Read-only view of the remaining undistributed balance in a stream.
    pub fn balance(env: Env, id: u64) -> i128 {
        let stream: Stream = env
            .storage()
            .persistent()
            .get(&DataKey::Stream(id))
            .expect("stream not found");
        stream.balance
    }

    fn next_id(env: &Env) -> u64 {
        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(0);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));
        id
    }
}

mod test;
