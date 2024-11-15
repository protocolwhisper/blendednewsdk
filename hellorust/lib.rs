#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;
extern crate fluentbase_sdk;

use alloc::string::ToString;
use alloc::string::String;
use fluentbase_sdk::{
    basic_entrypoint,
    derive::{router, function_id, Contract},
    SharedAPI,
};

#[derive(Contract)]
struct ROUTER<SDK> {
    sdk: SDK,
}

pub trait RouterAPI {
    fn greeting(&self) -> String;
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> RouterAPI for ROUTER<SDK> {
    #[function_id("greeting()")]
    fn greeting(&self) -> String {
        "Hello,".to_string()
    }
}

impl<SDK: SharedAPI> ROUTER<SDK> {
    fn deploy(&self) {
        // any custom deployment logic here
    }
}

basic_entrypoint!(ROUTER);

#[cfg(test)]
mod tests {
    use super::*;
    use fluentbase_sdk::{journal::JournaßlState, runtime::TestingContext};
    use hex_literal::hex;

    #[test]
    fn test_contract_method_works() {
        // form test input
        let input = hex!("f8194e480000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000e2248656c6c6f2c20576f726c6422000000000000000000000000000000000000");
        let _msg = greetingCall::abi_decode(&input, true).unwrap_or_else(|e| {
            panic!("Failed to decode input {:?} {:?}", "msg", e,);
        });
        let sdk = TestingContext::empty().with_input(input);
        // run router
        let mut greeting = ROUTER::new(JournalState::empty(sdk.clone()));
        greeting.deploy();
        greeting.main();
        // check result
        let test_output = sdk.take_output();
        assert_eq!(test_output,
                   hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000e2248656c6c6f2c20576f726c6422000000000000000000000000000000000000"
    ));
    }
}