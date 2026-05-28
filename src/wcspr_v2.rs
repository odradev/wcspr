use odra::{casper_types::{PublicKey, U256, bytesrepr::Bytes}, prelude::*};
use odra_modules::cep3009::CEP3009;
use crate::wcspr_v1::WCSPRV1;

/// The WCSPR V2 module with CEP3009.
#[odra::module()]
pub struct WCSPRV2 {
    cspr_v1: SubModule<WCSPRV1>,
    cep3009: SubModule<CEP3009>
}

/// The WCSPR V2 module implementation.
#[odra::module]
impl WCSPRV2 {
    /// Initializes the contract with the metadata.
    pub fn init(&mut self, chain_name: String) {
        self.cspr_v1.init();
        self.cep3009.init(chain_name);
    }

    /// Upgrades the contract. Initializes the CEP3009 module with the chain name.
    pub fn upgrade(&mut self, chain_name: String) {
        self.cep3009.init(chain_name);
    }

    delegate! {
        to self.cep3009 {
            fn authorization_state(&self, authorizer: Address, nonce: Bytes) -> bool;
            fn transfer_with_authorization(
                &mut self,
                from: Address,
                to: Address,
                amount: U256,
                valid_after: u64,
                valid_before: u64,
                nonce: Bytes,
                public_key: PublicKey,
                signature: Bytes
            );
            fn receive_with_authorization(
                &mut self,
                from: Address,
                to: Address,
                amount: U256,
                valid_after: u64,
                valid_before: u64,
                nonce: Bytes,
                public_key: PublicKey,
                signature: Bytes
            );
             fn cancel_authorization(
                &mut self,
                authorizer: Address,
                nonce: Bytes,
                public_key: PublicKey,
                signature: Bytes
            );
        }

        to self.cspr_v1 {
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn decimals(&self) -> u8;
            fn total_supply(&self) -> U256;
            fn balance_of(&self, address: &Address) -> U256;
            fn allowance(&self, owner: &Address, spender: &Address) -> U256;
            fn approve(&mut self, spender: &Address, amount: &U256);
            fn decrease_allowance(&mut self, spender: &Address, decr_by: &U256);
            fn increase_allowance(&mut self, spender: &Address, inc_by: &U256);
            fn transfer(&mut self, recipient: &Address, amount: &U256);
            fn transfer_from(&mut self, owner: &Address, recipient: &Address, amount: &U256);
            #[odra(payable)] fn deposit(&mut self);
            fn withdraw(&mut self, amount: &U256);
            fn withdraw_to(&mut self, recipient: &Address, amount: &U256);
        }
    }
}
