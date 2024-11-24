use num::traits::{CheckedAdd, CheckedSub, Zero,One};
use std::{collections::BTreeMap, ops::AddAssign};

// Generics -> ALlowing you to define custom types

#[derive(Debug)]

pub struct Pallet<AccountId, Nonce, Blocknumber> {
    nonce: BTreeMap<AccountId, Nonce>,
    block_number: Blocknumber,
}

impl<AccountId, Nonce, Blocknumber> Pallet<AccountId, Nonce, Blocknumber>
where
    AccountId: Ord+ Clone,
    Nonce: Ord + CheckedAdd + CheckedSub + Zero +One +Copy,
    Blocknumber: Zero + CheckedAdd + Copy+ CheckedSub+One+AddAssign,
{
    pub fn new() -> Self {
        Self {
            nonce: BTreeMap::new(),
            block_number: Blocknumber::zero(),
        }
    }
    pub fn block_number(&self) -> Blocknumber {
        self.block_number
    }

    pub fn get_nonce(&self, from: &AccountId) -> Nonce {
        *self.nonce.get(from).unwrap_or(&Nonce::zero())
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&Blocknumber::one()).expect("Overflow");
    }

    pub fn increment_nonce(&mut self, from: &AccountId) {
        let user_nonce = self.get_nonce(from);


        self.nonce.insert(from.clone(), user_nonce + Nonce::one());
    }
}
#[test]
fn init_systems() {
    let mut pallet:Pallet<String,u32,u32> = Pallet::new();
    let user1 = String::from("Alice");
    assert_eq!(pallet.block_number, 0);
    pallet.increment_block_number();

    assert_eq!(pallet.get_nonce(&user1), 0);
    assert_eq!(pallet.block_number, 1);
    pallet.increment_nonce(&user1);
    assert_eq!(pallet.get_nonce(&user1), 1);
}
