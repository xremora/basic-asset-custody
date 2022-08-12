/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

// default storage cost
// @todo
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

mod views;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub total_amount: u128,
    pub payments: UnorderedMap<AccountId, u128>, // {beneficiary:amount}
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            total_amount: 0,
            payments: UnorderedMap::new(b"p"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            total_amount: 0,
            payments: UnorderedMap::new(b"p"),
        }
    }

    // Public method - send payment to the beneficiary
    // returns the amount paid
    pub fn send_payment(&mut self, beneficiary: AccountId) -> U128 {
        // amount sent
        let amount: Balance = env::attached_deposit();
        self.payments.insert(&beneficiary, &amount);
        self.total_amount += amount;
        // emit events(logs)
        log!(
            "Payment of amount: {} sent to beneficiary: {} in escrow",
            amount,
            beneficiary
        );
        return U128(amount);
    }

    // Public m&&ethod - accepts a greeting, such as "howdy", and records it
    pub fn withdraw_payment(&mut self, amount: u128) -> U128 {
        // log!("Amount: {} withdrawn from escrow  by: {}", amount, beneficiary);

        // extract the caller
        let caller: AccountId = env::predecessor_account_id();
        // assert that the caller has enough balance to withdraw
        let _amount = self.payments.get(&caller).unwrap_or(0);
        assert!(amount == _amount, "amount mismatch! ");

        // delete the entry
        self.payments.remove(&caller);
        self.total_amount -= amount;
        // Promise::new(caller.clone()).transfer(amount);
        Promise::new(caller).transfer(amount);
        return U128(amount);
    }
}

/*
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    const BENEFICIARY: &str = "beneficiary";
    const BENEFICIARY2: &str = "beneficiary2";
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn initializes() {
        let contract = Contract::new();
        assert_eq!(contract.total_amount, 0)
    }

    #[test]
    fn send_payment() {
        let mut contract = Contract::new();

        // Make a payment
        set_context("caller_a", 1 * NEAR);
        contract.send_payment(BENEFICIARY.parse().unwrap());

        let sent_amount = contract.get_balanceof(BENEFICIARY.parse().unwrap());


        // Check the donation was recorded correctly
        assert_eq!(sent_amount.amount.0, 1 * NEAR);

        // Make another donation
        set_context("caller2", 2 * NEAR);
        contract.send_payment(BENEFICIARY2.parse().unwrap());
        let sent_amount2 = contract.get_balanceof(BENEFICIARY2.parse().unwrap());

        // Check the donation was recorded correctly
        assert_eq!(sent_amount2.amount.0, 2 * NEAR);
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

}
