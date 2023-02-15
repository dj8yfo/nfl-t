use engine::{Account, Ledger};
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use super::{Event, Feed};

const EVENT_INTERVAL_SECS: u64 = 2;

pub fn start_oracle_routine(
    account: Account,
    ledger: Arc<Mutex<Box<dyn Ledger + Send>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let external_data: Event = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let tx = Feed::data_tx(external_data, account.clone());
        ledger.lock().expect("lock poisoned").process_tx(tx);
        thread::sleep(Duration::new(EVENT_INTERVAL_SECS, 0));
    })
}
