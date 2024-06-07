#![no_std]
extern crate alloc;
use alloy_sol_types::{sol, SolValue};
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env};

#[cfg(test)]
mod test;

#[contracterror]
#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Decode = 1,
}

#[contract]
pub struct Contract;

// sol! {
//     struct Input {
//         bytes32 a;
//         uint256 b;
//         uint256 c;
//     }
//     struct Output {
//         bytes32 a;
//         uint256 r;
//     }
// }

#[contractimpl]
impl Contract {
    pub fn exec(e: &Env, input: Bytes) -> Result<Bytes, Error> {
        // let mut input_buf = [0u8; 128];
        // let mut input_slice = &mut input_buf[..input.len() as usize];
        // input.copy_into_slice(&mut input_slice);

        x.map_err(|y| y + 1)?;
        // let output = Output {
        //     a: input.a,
        //     r: input.b + input.c,
        // };
        // let output_encoded = output.abi_encode();
        // Ok(Bytes::from_slice(e, &output_encoded))

        // 10
    }
}
