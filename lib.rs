#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use ink_prelude::string::String;
    //use openbrush::traits::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::ownable::*,
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22 for Contract {}

    impl Ownable for Contract {}
    
    impl PSP22Metadata for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
                let caller: AccountId = instance.env().caller();
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = decimal;
                instance._init_with_owner(caller);
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint total_supply");
            })
        }


        #[ink(message, payable)]
        #[openbrush::modifiers(only_owner)]
        pub fn withdraw_specific_amount(&mut self, value: Balance) {
            
            assert!(value <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "requested transfer failed. insufficient free funds or contract balance maybe bought too low"
                )
            }
        }


    }

}