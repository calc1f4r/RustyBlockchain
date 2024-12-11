mod balances;
mod system;
mod types{
   pub  type AccountId=String;
    pub type BlockNumber=u32;
    pub type Nonce=u32;
    pub type Balance=u128;
}
impl system::Config for Runtime{
    type AccountId=types::AccountId;
    type BlockNumber=types::BlockNumber;
    type Nonce=types::Nonce;    
}
impl balances::Config for Runtime{
    type Balance = types::Balance;
}
pub struct Runtime{ 
    system:system::Pallet<Runtime>,
    balances:balances::Pallet<Runtime>,
}

impl Runtime{
    fn new()->Self{
        Self{
            system:system::Pallet::new(),
            balances:balances::Pallet::new()
        }
    }
}

fn main() {
    let mut runtime=Runtime::new();
    let alice="Alice".to_string();
    let bob="bob".to_string();
    let charlie="charlie".to_string();
    

    runtime.balances.set_balances(&alice, 1000);
    
    runtime.system.increment_block_number();

    assert_eq!(runtime.system.get_block_number(), 1);
    
    let from = alice.clone();
    let to = bob.clone();
    let amount = 500;

    match runtime.balances.transfer(&from, &to, amount) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    assert_eq!(runtime.balances.get_balance(&alice), 500);
    assert_eq!(runtime.balances.get_balance(&bob), 500);

}
