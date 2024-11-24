// making them useful
mod balances;

mod system;
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
}
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<types::AccountId,types::Nonce,types::BlockNumber>,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = String::from("Alice");
    let bob: String = String::from("Bob");
    let charlie = String::from("Charlie");
    runtime.balances.set_balance(&alice, 100);
    runtime.system.increment_nonce(&alice);

    runtime.system.increment_block_number();

    let _ = runtime.balances.transfer(&alice, &bob, 10).map_err(|err| {
        eprintln!("Error: {:?}", err);
    });
    let _ = runtime
        .balances
        .transfer(&alice, &charlie, 1)
        .map_err(|err| {
            eprintln!("Error: {:?}", err);
        });
    println!("{:#?}", runtime);
}
