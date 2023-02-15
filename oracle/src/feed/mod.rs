use console::style;
use engine::{Account, Pallet, PalletDescriptor};
use serde::{Deserialize, Serialize};

pub mod routine;

pub mod api;
pub mod tx;

pub struct Feed;

pub type Event = Vec<u8>;

#[derive(Serialize, Deserialize)]
pub struct State {
    body: Vec<Event>,
}

impl State {
    pub fn last(&self) -> Option<Event> {
        self.body.last().cloned()
    }
}

pub static PALLET_DESCRIPTOR: &str = "ORACLE_FEED";
const MAX_EVENTS: usize = 100;

impl Pallet for Feed {
    fn process_tx(&mut self, tx: &engine::RawTx, storage: &mut dyn engine::LedgerState) {
        let sender: (PalletDescriptor, Account) = tx.from.clone();
        if sender.0 != PALLET_DESCRIPTOR {
            return;
        }
        let new_event = tx.payload.clone();
        let digest = format!("{:x}", md5::compute(tx.payload.clone()));
        let length: usize;
        let state = Feed::events(sender.1.clone(), storage);
        match state {
            Some(mut state) => {
                debug_assert!(
                    state.body.len() <= MAX_EVENTS,
                    "event feed must not be longer than {}",
                    MAX_EVENTS
                );

                if state.body.len() == MAX_EVENTS {
                    state.body.remove(0);
                }
                state.body.push(new_event);

                length = state.body.len();
                let encoded: Vec<u8> = bincode::serialize(&state).unwrap();
                storage.set(sender, encoded);
            }
            None => {
                let initial = State {
                    body: vec![new_event],
                };

                length = 1;
                let encoded: Vec<u8> = bincode::serialize(&initial).unwrap();
                storage.set(sender, encoded);
            }
        }

        println!(
            "{}",
            style(format!("pushed new event {}, total {}", digest, length)).red()
        );
    }
}
