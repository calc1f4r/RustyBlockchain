use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};


pub struct Pallet<AccountId,Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl <AccountId,Balance>  Pallet<AccountId,Balance> where 
AccountId:Ord+Clone,
Balance:Zero+CheckedSub+CheckedAdd+Copy+Clone
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    } 
    pub fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), String> {
        let balance_from = *self.balances.get(from).unwrap_or(&Balance::zero());
        let balance_to = *self.balances.get(to).unwrap_or(&Balance::zero());

        let new_balance_from = balance_from.checked_sub(&amount).ok_or("Insufficient Balance".to_string())?;
        let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow".to_string())?;

        self.balances.insert(from.clone(), new_balance_from);
        self.balances.insert(to.clone(), new_balance_to);
        Ok(())
    }

    pub fn set_balances(&mut self,from:&AccountId,amount: Balance)->Result<(),String>{
        self.balances.insert(from.clone(), amount);
        Ok(())
    }

    pub fn get_balance(&self, user: &AccountId) -> Balance     {
        return *self.balances.get(user).unwrap_or(&Balance::zero());
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_pallet() {
        let mut pallet = Pallet::new();
        assert!(pallet.balances.is_empty());

        pallet.balances.insert("Alice".to_string(), 1000);
        assert_eq!(pallet.balances.get(&"Alice".to_string()), Some(&1000));
    }
    #[test]
    fn test_transfer_pallet() {
        let mut pallet = Pallet::new();
        pallet.balances.insert("Alice".to_string(), 1000);
        pallet.balances.insert(String::from("Bob"), 100);
        assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 100).is_ok());
    }

    #[test]
    fn test_get_balance() {
        let mut pallet = Pallet::new();
        pallet.balances.insert("Alice".to_string(), 1000);
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 1000);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
    }
}