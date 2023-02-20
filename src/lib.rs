#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

const COUNTER: Symbol = symbol!("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.
        count += 1;
        env.storage().set(&COUNTER, &count);
        env.events().publish((COUNTER, symbol!("increment")), count);
        count
    }
}

#[cfg(test)]
mod test;
