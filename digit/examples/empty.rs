#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct EmptyContract;

#[contractimpl]
impl EmptyContract {
}
