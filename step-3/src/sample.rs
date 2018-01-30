#![no_std]
#![allow(non_snake_case)]
#![feature(alloc)]
#![feature(proc_macro)]
#![feature(wasm_import_memory)]
#![wasm_import_memory]

extern crate parity_hash;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate alloc;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
/// Bigint used for 256-bit arithmetic
extern crate bigint;

pub mod token {
    use pwasm_ethereum;
    use parity_hash::H256;
    use bigint::U256;

	// eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;
    use alloc::Vec;

    static TOTAL_SUPPLY_KEY: H256 = H256([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenContract {
		/// The constructor
        fn constructor(&mut self, _total_supply: U256);
        /// Total amount of tokens
        #[constant]
        fn totalSupply(&mut self) -> U256;
    }

    pub struct TokenContractInstance;

    impl TokenContract for TokenContractInstance {
        fn constructor(&mut self, total_supply: U256) {
            // Set up the total supply for the token
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
        }

        fn totalSupply(&mut self) -> U256 {
            pwasm_ethereum::read(&TOTAL_SUPPLY_KEY).into()
        }
    }
}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

/// The main function receives a pointer for the call descriptor.
#[no_mangle]
pub fn call(desc: *mut u8) {
    let (args, result) = unsafe { pwasm_std::parse_args(desc) };
    let mut endpoint = token::TokenEndpoint::new(token::TokenContractInstance{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    result.done(endpoint.dispatch(&args));
}

#[no_mangle]
pub fn deploy(desc: *mut u8) {
    let (args, _) = unsafe { pwasm_std::parse_args(desc) };
    let mut endpoint = token::TokenEndpoint::new(token::TokenContractInstance{});
    endpoint.dispatch_ctor(&args);
}
