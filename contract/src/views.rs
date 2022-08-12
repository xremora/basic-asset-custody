use near_sdk::serde::Serialize;

use crate::*;



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Balance { // amuount that can be viewed by the account
    pub account_id: AccountId,
    pub amount: U128,
}


#[near_bindgen]
impl Contract {
    // get balance of beneficiary
    pub fn get_balanceof(&self, account_id: AccountId) -> Balance {
        Balance {
            account_id: account_id.clone(),
            amount: U128(self.payments.get(&account_id).unwrap_or(0)),
        }
    }
    pub fn total_payments(&self) -> u128 {
        self.total_amount
    }

    //paginated view of balances
    pub fn get_balances(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Balance> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.payments
            .keys()
            // skip to start
            .skip(start as usize)
            // take the first `limit` elements in the vec
            .take(limit.unwrap_or(50) as usize)
            .map(|account| self.get_balanceof(account))
            // since we turned map into an itertor, we need to turn it back to vec to return
            .collect()
    }
}
