mod balances;
mod system;
mod support;

use crate::support::Dispatch;



pub mod types{
use crate::support;

   pub  type AccountId=String;
    pub type BlockNumber=u32;
    pub type Nonce=u32;
    pub type Balance=u128;

    // Define the extrinsic type 
    pub type Extrinsic=support::Extrinsic<AccountId,crate::RuntimeCall>;

    pub type Header=support::Header<BlockNumber>;
    pub type Block=support::Block<Header,Extrinsic>;

}
pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}
impl system::Config for Runtime{
    type AccountId=types::AccountId;
    type BlockNumber=types::BlockNumber;
    type Nonce=types::Nonce;    
}
impl balances::Config for Runtime{
    type Balance = types::Balance;
}

#[derive(Debug)]
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
    fn execute_block(&mut self, block:types::Block)->support::DispatchResult{
        self.system.increment_block_number();

        if  self.system.get_block_number()!=block.header.block_number{
            return Err("Block number mismatch")
        }
        for (i,support::Extrinsic{caller   ,call}) in block.extrinsics.into_iter().enumerate(){
            self.system.increment_nonce(caller.clone());
            let _=self.dispatch(caller,call).map_err(|e|{eprintln!("Extrinsic Error \n\t Block Number : {}\n\t Extrinsic Number:{}\n\t Error:{}",block.header.block_number,i,e)});
        }
        Ok(())
    }
}
impl crate::support::Dispatch for Runtime{
    type Caller = <Runtime as system::Config>::AccountId;
    type Call=RuntimeCall;
    
    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> support::DispatchResult {
        match runtime_call {
	RuntimeCall::BalancesTransfer { to, amount } => {
		self.balances.transfer(&caller, &to, amount);   
	}
    }   
        Ok(())
    }
}
fn main() {
    let mut runtime=Runtime::new();

    let alice="Alice".to_string();
    let bob="Bob".to_string();
    let charlie="Charlie".to_string();
    runtime.balances.set_balances(&alice, 10000);

    let block_1=types::Block{
        header:support::Header { block_number: 1 },
        extrinsics:vec![
            support::Extrinsic{
                caller:alice.clone(),
                call:RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 1000 }
            },
            support::Extrinsic{
                caller:alice.clone(),
                call:RuntimeCall::BalancesTransfer { to: charlie.clone(), amount: 1000 }
            },
            
        ]
    };

    runtime.execute_block(block_1);
    print!("{:#?}",runtime);
    // let alice="Alice".to_string();
    // let bob="bob".to_string();
    // let charlie="charlie".to_string();
    

    // runtime.balances.set_balances(&alice, 1000);
    
    // runtime.system.increment_block_number();

    // assert_eq!(runtime.system.get_block_number(), 1);
    
    // let from = alice.clone();
    // let to = bob.clone();
    // let amount = 500;

    // match runtime.balances.transfer(&from, &to, amount) {
    //     Ok(_) => println!("Transfer successful"),
    //     Err(e) => println!("Transfer failed: {}", e),
    // }

    // assert_eq!(runtime.balances.get_balance(&alice), 500);
    // assert_eq!(runtime.balances.get_balance(&bob), 500);

}
