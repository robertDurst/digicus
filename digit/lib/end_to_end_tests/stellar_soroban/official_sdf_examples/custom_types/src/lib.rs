#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: i64,
    pub last_incr: i64,
}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, incr: i64) -> i64 {
        let mut state = Self::get_state(env.clone());
        state.count = state.count + incr;
        let mut ASSIGN_EXPRESSION_LEFT = state.last_incr;
        env.storage().instance().set(&STATE, &state);
        let mut Thing_to_return = state.count;
        Thing_to_return
    }

    pub fn get_state(env: Env) -> State {
        let mut METHOD_CALL_ARG_1_0 = State {
            count: 0,
            last_incr: 0,
        };
        let mut Thing_to_return = env
            .storage()
            .instance()
            .get(&STATE)
            .unwrap_or(METHOD_CALL_ARG_1_0);
        Thing_to_return
    }
}

mod test;
