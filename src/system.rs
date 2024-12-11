use std::collections::BTreeMap;
use num::{CheckedAdd, One, Zero};

pub struct Pallet<Nonce, BlockNumber, AccountId> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<Nonce, BlockNumber, AccountId> Pallet<Nonce, BlockNumber, AccountId>
where
    BlockNumber: CheckedAdd + Zero + One + Copy,
    AccountId: Ord + Clone,
    Nonce: Zero + CheckedAdd + One + Copy,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn increment_block_number(&mut self) -> Result<(), String> {
        self.block_number.checked_add(&BlockNumber::one());
        Ok(())
    }

    pub fn increment_nonce(&mut self, from: AccountId) -> Result<(), String> {
        let current_nonce_user = self.nonce.get(&from).unwrap_or(&Nonce::zero()).clone();
        let new_nonce_user = current_nonce_user.checked_add(&Nonce::one()).ok_or("Overflow")?;
        self.nonce.insert(from.clone(), new_nonce_user);
        Ok(())
    }

    pub fn get_block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn get_nonce(&self, from: &AccountId) -> Nonce {
        self.nonce.get(from).unwrap_or(&Nonce::zero()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation_new_system() {
        let pallet: Pallet<u32, u32, String> = Pallet::new();
        assert_eq!(pallet.block_number, 0);
        assert!(pallet.nonce.is_empty());
    }

    #[test]
    fn test_increment_block_number() {
        let mut pallet: Pallet<u32, u32, String> = Pallet::new();
        assert_eq!(pallet.block_number, 0);
        pallet.increment_block_number().unwrap();
        assert_eq!(pallet.block_number, 1);
    }

    #[test]
    fn test_increment_nonce() {
        let mut pallet: Pallet<u32, u32, String> = Pallet::new();
        let user = String::from("user1");
        assert_eq!(*pallet.nonce.get(&user).unwrap_or(&0), 0);
        pallet.increment_nonce(user.clone()).unwrap();
        assert_eq!(*pallet.nonce.get(&user).unwrap_or(&0), 1);
    }

    #[test]
    fn test_increment_nonce_overflow() {
        let mut pallet: Pallet<u32, u32, String> = Pallet::new();
        let user = String::from("user1");
        pallet.nonce.insert(user.clone(), u32::MAX);
        let result = pallet.increment_nonce(user.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Overflow");
    }

    #[test]
    fn test_get_block_number() {
        let mut pallet: Pallet<u32, u32, String> = Pallet::new();
        assert_eq!(pallet.get_block_number(), 0);
        pallet.increment_block_number().unwrap();
        assert_eq!(pallet.get_block_number(), 1);
    }

    #[test]
    fn test_get_nonce() {
        let mut pallet: Pallet<u32, u32, String> = Pallet::new();
        let user = String::from("user1");
        assert_eq!(pallet.get_nonce(&user), 0);
        pallet.increment_nonce(user.clone()).unwrap();
        assert_eq!(pallet.get_nonce(&user), 1);
    }
}
