// making them useful
mod balances;


mod system;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
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

    let alice=String::from("Alice");
    let bob:String=String::from("Bob");
    let charlie=String::from("Charlie");
    runtime.balances.set_balance(&alice, 100);
    runtime.system.increment_nonce(&alice);

    runtime.system.increment_block_number();

    let _=runtime.balances.transfer(&alice,&bob, 10).map_err(|err| {
        eprintln!("Error: {:?}", err);
    });
    let _=runtime.balances.transfer(&alice,&charlie, 1).map_err(|err| {
        eprintln!("Error: {:?}", err);
    });
    println!("{:#?}",runtime);

}
