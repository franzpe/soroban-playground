#![cfg(test)]
use crate::{
    Error, IncrementContract, IncrementContractClient, IncrementContractCustomTypes,
    IncrementContractCustomTypesClient, State,
};

use super::{Contract, ContractClient};
use soroban_sdk::{
    symbol,
    testutils::{Address as _, Events, Logger},
    vec, Address, Env, IntoVal,
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol!("Dev"));

    let logs = env.logger().all();
    std::println!("{}", logs.join("\n"));

    assert_eq!(logs, std::vec!["Hello Symbol(Dev)"]);
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}

#[test]
fn test_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}

#[test]
fn test_custom_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContractCustomTypes);
    let client = IncrementContractCustomTypesClient::new(&env, &contract_id);

    assert_eq!(client.incr_state(&1), 1);
    assert_eq!(client.incr_state(&10), 11);
    assert_eq!(
        client.get_state(),
        State {
            count: 11,
            last_incr: 10
        }
    );
}

#[test]
fn test_increment_w_max() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.try_incr_max(), Ok(Ok(1)));
    assert_eq!(client.try_incr_max(), Ok(Ok(2)));
    assert_eq!(client.try_incr_max(), Ok(Ok(3)));
    assert_eq!(client.try_incr_max(), Ok(Ok(4)));
    assert_eq!(client.try_incr_max(), Ok(Ok(5)));
    assert_eq!(client.try_incr_max(), Err(Ok(Error::LimitReached)));
}

#[test]
fn test_increment_w_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    assert_eq!(client.incr_auth(&user_1, &5), 5);

    // Verify that the user indeed had to authorize a call of `increment` with
    // the expected arguments:
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            // Address for which auth is performed
            user_1.clone(),
            // Identifier of the called contract
            contract_id.clone(),
            // Name of the called function
            symbol!("incr_auth"),
            // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
            (user_1.clone(), 5_u32).into_val(&env)
        )]
    );

    // Do more `increment` calls. It's not necessary to verify authorizations
    // for every one of them as we don't expect the auth logic to change from
    // call to call.
    assert_eq!(client.incr_auth(&user_1, &2), 7);
    assert_eq!(client.incr_auth(&user_2, &1), 1);
    assert_eq!(client.incr_auth(&user_1, &3), 10);
    assert_eq!(client.incr_auth(&user_2, &4), 5);
}
