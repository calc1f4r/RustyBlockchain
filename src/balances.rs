
use std::collections::BTreeMap;
use num::traits::{CheckedAdd,CheckedSub};
type AccountId=String;
type Balance=u128;


#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<AccountId, Balance>,
}
impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, to: &AccountId, amount: u128) -> Option<Balance> {
        self.balances.insert(to.clone(), amount)
    }

    pub fn balance(&self, from: &AccountId) -> Balance {
        *self.balances.get(from).unwrap_or(&0)
    }
    pub fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), String> {
        let balance_from = self.balance(from);
        let balance_to = self.balance(to);
        if amount > balance_from {
            return Err("Recipient does not have enough balance in his account".to_string());
        }

        self.set_balance(from, balance_from - amount);
        self.set_balance(to, amount + balance_to);
        Ok(())
    }
}
#[test]
fn init_balances() {
    let mut pallet = Pallet::new();
    let person1 = AccountId::from("Alice");
    let person2 = AccountId::from("Bob");

    let balance_person1: u128 = 23;
    let balance_person2: u128 = 45;

    pallet.set_balance(&person1, balance_person1);
    pallet.set_balance(&person2, balance_person2);

    // passing the reference to the function
    assert_eq!(pallet.balance(&person1), balance_person1);

    // passing the reference to the function
    assert_eq!(pallet.balance(&person2), balance_person2);
}
#[test]
fn test_transfer() {
    let mut pallet = Pallet::new();

    let person1 = AccountId::from("Alice");
    let person2 = AccountId::from("Bob");

    let balance_person1: u128 = 23;
    let balance_person2: u128 = 45;

    pallet.set_balance(&person1, balance_person1);
    pallet.set_balance(&person2, balance_person2);

    pallet.transfer(&person1, &person2, 2).unwrap();

    assert_eq!(pallet.balance(&person1), balance_person1 - 2);
    assert_eq!(pallet.balance(&person2), balance_person2 + 2);
}
