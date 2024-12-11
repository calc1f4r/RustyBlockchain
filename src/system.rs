use std::collections::BTreeMap;
use num::{CheckedAdd, One, Zero};




pub trait Config{
    type BlockNumber:CheckedAdd + Zero + One + Copy;
    type AccountId:Ord + Clone;
    type Nonce:Zero + CheckedAdd + One + Copy;
}



pub struct Pallet<T:Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T:Config> Pallet<T>
where
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn increment_block_number(&mut self) -> Result<(), String> {
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).ok_or("Overflow")?;
        Ok(())
    }

    pub fn increment_nonce(&mut self, from: T::AccountId) -> Result<(), String> {
        let current_nonce_user = self.nonce.get(&from).unwrap_or(&T::Nonce::zero()).clone();
        let new_nonce_user = current_nonce_user.checked_add(&T::Nonce::one()).ok_or("Overflow")?;
        self.nonce.insert(from.clone(), new_nonce_user);
        Ok(())
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn get_nonce(&self, from: &T::AccountId) -> T::Nonce {
        self.nonce.get(from).unwrap_or(&T::Nonce::zero()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type BlockNumber = u32;
        type AccountId = String;
        type Nonce = u32;
    }

    #[test]
    fn test_creation_new_system() {
        let pallet: Pallet<TestConfig> = Pallet::new();
        assert_eq!(pallet.block_number, 0);
        assert!(pallet.nonce.is_empty());
    }

    #[test]
    fn test_increment_block_number() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        assert_eq!(pallet.block_number, 0);
        pallet.increment_block_number().unwrap();
        assert_eq!(pallet.block_number, 1);
    }

    #[test]
    fn test_increment_nonce() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let user = String::from("user1");
        assert_eq!(*pallet.nonce.get(&user).unwrap_or(&0), 0);
        pallet.increment_nonce(user.clone()).unwrap();
        assert_eq!(*pallet.nonce.get(&user).unwrap_or(&0), 1);
    }

    #[test]
    fn test_increment_nonce_overflow() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let user = String::from("user1");
        pallet.nonce.insert(user.clone(), u32::MAX);
        let result = pallet.increment_nonce(user.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Overflow");
    }

    #[test]
    fn test_get_block_number() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        assert_eq!(pallet.get_block_number(), 0);
        pallet.increment_block_number().unwrap();
        assert_eq!(pallet.get_block_number(), 1);
    }

    #[test]
    fn test_get_nonce() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        let user = String::from("user1");
        assert_eq!(pallet.get_nonce(&user), 0);
        pallet.increment_nonce(user.clone()).unwrap();
        assert_eq!(pallet.get_nonce(&user), 1);
    }
}
