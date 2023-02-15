use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use engine::simple_ledger::MemoryLedger;
use engine::{Ledger, Pallet};

use oracle::feed;

use user_pallet::OracleUser;

const USER_ACTION_INTERVAL_SECS: u64 = 10;
fn main() {
    let mut pallets = HashMap::new();

    let feed: Box<dyn Pallet + Send> = Box::new(feed::Feed);
    let user: Box<dyn Pallet + Send> = Box::new(OracleUser);
    pallets.insert(feed::PALLET_DESCRIPTOR.to_string(), feed);
    pallets.insert(user_pallet::PALLET_DESCRIPTOR.to_string(), user);

    let l: Box<dyn Ledger + Send> = Box::new(MemoryLedger::new(pallets));
    let shared_ledger = Arc::new(Mutex::new(l));

    let oracle_from =
        "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_string();
    let jh = feed::routine::start_oracle_routine(oracle_from.clone(), shared_ledger.clone());

    let user_acc = "0xdeadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeafdeadbeaf".to_string();
    let user_jh = thread::spawn(move || {
        let tx = OracleUser::init_tx(user_acc.clone(), oracle_from);
        shared_ledger.lock().expect("lock poisoned").process_tx(tx);
        loop {
            thread::sleep(Duration::new(USER_ACTION_INTERVAL_SECS, 0));
            let watch = OracleUser::watch_tx(user_acc.clone(), vec![]);
            shared_ledger
                .lock()
                .expect("lock poisoned")
                .process_tx(watch);
        }
    });
    user_jh.join().unwrap();

    jh.join().unwrap();
}
