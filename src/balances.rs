use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

// Define a trait Config that inherits from crate::system::Config
// It requires a Balance type that supports Zero, CheckedSub, CheckedAdd, Copy, and Clone
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Clone;
}

// Define the Pallet struct with a generic type T that implements the Config trait
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    // Constructor to create a new Pallet instance with an empty balances map
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Method to transfer an amount from one account to another
    pub fn transfer(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), String> {
        // Get the balance of the 'from' account, defaulting to zero if not found
        let balance_from = *self.balances.get(from).unwrap_or(&T::Balance::zero());
        // Get the balance of the 'to' account, defaulting to zero if not found
        let balance_to = *self.balances.get(to).unwrap_or(&T::Balance::zero());

        // Subtract the amount from the 'from' account, returning an error if it would underflow
        let new_balance_from = balance_from.checked_sub(&amount).ok_or("Insufficient Balance".to_string())?;
        // Add the amount to the 'to' account, returning an error if it would overflow
        let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow".to_string())?;

        // Update the balances map with the new balances
        self.balances.insert(from.clone(), new_balance_from);
        self.balances.insert(to.clone(), new_balance_to);
        Ok(())
    }

    // Method to set the balance of a specific account
    pub fn set_balances(&mut self, from: &T::AccountId, amount: T::Balance) -> Result<(), String> {
        self.balances.insert(from.clone(), amount);
        Ok(())
    }

    // Method to get the balance of a specific account
    pub fn get_balance(&self, user: &T::AccountId) -> T::Balance {
        *self.balances.get(user).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system;

    // Define a test configuration struct
    struct TestConfig;

    // Implement the system::Config trait for the test configuration
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    // Implement the Config trait for the test configuration
    impl Config for TestConfig {
        type Balance = u64;
    }

    #[test]
    fn test_new_pallet() {
        // Create a new Pallet instance
        let mut pallet = Pallet::<TestConfig>::new();
        // Assert that the balances map is initially empty
        assert!(pallet.balances.is_empty());

        // Insert a balance for Alice and assert that it is correctly stored
        pallet.balances.insert("Alice".to_string(), 1000);
        assert_eq!(pallet.balances.get(&"Alice".to_string()), Some(&1000));
    }

    #[test]
    fn test_transfer_pallet() {
        // Create a new Pallet instance
        let mut pallet  = Pallet::<TestConfig>::new();
        // Insert initial balances for Alice and Bob
        pallet.balances.insert("Alice".to_string(), 1000);
        pallet.balances.insert("Bob".to_string(), 100);
        // Perform a transfer from Alice to Bob and assert that it succeeds
        assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 100).is_ok());
        // Assert that the balances have been updated correctly
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 900);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 200);
    }

    #[test]
    fn test_get_balance() {
        // Create a new Pallet instance
        let mut pallet = Pallet::<TestConfig>::new();
        // Insert a balance for Alice
        pallet.balances.insert("Alice".to_string(), 1000);
        // Assert that the balance for Alice is correct
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 1000);
        // Assert that the balance for Bob is zero (default)
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
    }
}