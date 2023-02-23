#![no_std]
use soroban_sdk::{contracterror, contractimpl, contracttype, log, symbol, vec, Env, Symbol, Vec};

/**
 * Hello world contract
 */
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        log!(&env, "Hello {}", to);
        vec![&env, symbol!("Hello"), to]
    }
}

/**
 * Increment contract with event
 */
pub struct IncrementContract;

const COUNTER: Symbol = symbol!("COUNTER");
const MAX: u32 = 5;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}

#[contractimpl]
impl IncrementContract {
    pub fn incr_max(env: Env) -> Result<u32, Error> {
        let mut count: u32 = Self::get_count(env.clone());
        count += 1;

        if count <= MAX {
            env.storage().set(&COUNTER, &count);
            Ok(count)
        } else {
            Err(Error::LimitReached)
        }
    }

    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = Self::get_count(env.clone());
        count += 1;
        env.storage().set(&COUNTER, &count);
        env.events().publish((COUNTER, symbol!("increment")), count);
        count
    }

    pub fn get_count(env: Env) -> u32 {
        env.storage().get(&COUNTER).unwrap_or(Ok(0)).unwrap()
    }
}

#[cfg(test)]
mod test;

/**
 * Increment contract with custom types
 */
pub struct IncrementContractCustomTypes;

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

const STATE: Symbol = symbol!("STATE");

#[contractimpl]
impl IncrementContractCustomTypes {
    pub fn incr_state(env: Env, incr: u32) -> u32 {
        let mut state = Self::get_state(env.clone());
        state.count += incr;
        state.last_incr = incr;
        env.storage().set(&STATE, &state);
        state.count
    }

    pub fn get_state(env: Env) -> State {
        env.storage()
            .get(&STATE)
            .unwrap_or_else(|| Ok(State::default()))
            .unwrap()
    }
}
