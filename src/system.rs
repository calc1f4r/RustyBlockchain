use 
std::collections::BTreeMap;


// Generics -> ALlowing you to define custom types
type AccountId=String;
type Blocknumber=u32;
type Nonce=u32;

#[derive(Debug)]

pub struct Pallet {
    nonce: BTreeMap<String, Nonce>,
    block_number: Blocknumber,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            nonce: BTreeMap::new(),
            block_number: 0,
        }
    }
    pub fn block_number(&self) -> Blocknumber {
        self.block_number
    }

    pub fn get_nonce(&self, from: &AccountId) -> Nonce {
        *self.nonce.get(from).unwrap_or(&0)
    }

    pub fn increment_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn increment_nonce(&mut self, from: &AccountId) {
        let user_nonce = self.get_nonce(from);
        self.nonce.insert(from.clone(), user_nonce + 1);
    }
}
#[test]
fn init_systems() {
    let mut pallet = Pallet::new();
    let user1 = String::from("Alice");
    assert_eq!(pallet.block_number, 0);
    pallet.increment_block_number();

    assert_eq!(pallet.get_nonce(&user1), 0);
    assert_eq!(pallet.block_number, 1);
    pallet.increment_nonce(&user1);
    assert_eq!(pallet.get_nonce(&user1), 1);
}
