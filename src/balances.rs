use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};


// Inheritance of configs
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Clone;
}

pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn transfer(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), String> {
        let balance_from = *self.balances.get(from).unwrap_or(&T::Balance::zero());
        let balance_to = *self.balances.get(to).unwrap_or(&T::Balance::zero());

        let new_balance_from = balance_from.checked_sub(&amount).ok_or("Insufficient Balance".to_string())?;
        let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow".to_string())?;

        self.balances.insert(from.clone(), new_balance_from);
        self.balances.insert(to.clone(), new_balance_to);
        Ok(())
    }

    pub fn set_balances(&mut self, from: &T::AccountId, amount: T::Balance) -> Result<(), String> {
        self.balances.insert(from.clone(), amount);
        Ok(())
    }

    pub fn get_balance(&self, user: &T::AccountId) -> T::Balance {
        *self.balances.get(user).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system;

    impl system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl Config for TestConfig {
        type Balance = u64;
    }

    struct TestConfig;

    #[test]
    fn test_new_pallet() {
        let mut pallet = Pallet::<TestConfig>::new();
        assert!(pallet.balances.is_empty());

        pallet.balances.insert("Alice".to_string(), 1000);
        assert_eq!(pallet.balances.get(&"Alice".to_string()), Some(&1000));
    }

    #[test]
    fn test_transfer_pallet() {
        let mut pallet = Pallet::<TestConfig>::new();
        pallet.balances.insert("Alice".to_string(), 1000);
        pallet.balances.insert("Bob".to_string(), 100);
        assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 100).is_ok());
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 900);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 200);
    }

    #[test]
    fn test_get_balance() {
        let mut pallet = Pallet::<TestConfig>::new();
        pallet.balances.insert("Alice".to_string(), 1000);
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 1000);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
    }
}