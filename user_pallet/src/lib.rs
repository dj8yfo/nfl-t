use console::style;
use engine::{Account, Pallet, PalletDescriptor};
use oracle::feed::{Event, Feed};
use serde::{Deserialize, Serialize};
use tx::TxPayload;

pub mod api;
pub mod tx;

pub struct OracleUser;

pub static PALLET_DESCRIPTOR: &str = "ORACLE_USER";

#[derive(Serialize, Deserialize)]
pub struct State {
    oracle: Account,
    last_observed_oracle_event: Option<Event>,
}

impl Pallet for OracleUser {
    fn process_tx(&mut self, tx: &engine::RawTx, storage: &mut dyn engine::LedgerState) {
        let sender: (PalletDescriptor, Account) = tx.from.clone();
        if sender.0 != PALLET_DESCRIPTOR {
            return;
        }

        let user = sender.1.clone();
        let payload: TxPayload = bincode::deserialize(&tx.payload).unwrap();
        let state: Option<State> = OracleUser::state(user.clone(), storage);
        match payload {
            TxPayload::Initialize { oracle_account } => {
                let updated = State {
                    oracle: oracle_account.clone(),
                    last_observed_oracle_event: None,
                };

                let encoded: Vec<u8> = bincode::serialize(&updated).unwrap();
                storage.set(sender, encoded);

                println!(
                    "{}",
                    style(format!(
                        "initialized {} user account to watch {} oracle feed",
                        user, oracle_account
                    ))
                    .cyan()
                );
            }
            TxPayload::Watch => {
                match state {
                    None => { // noop
                    }
                    Some(state) => {
                        let oracle = state.oracle;
                        let events = Feed::events(oracle.clone(), storage);
                        let last_event = events.and_then(|events| events.last());
                        let updated = State {
                            oracle: oracle.clone(),
                            last_observed_oracle_event: last_event.clone(),
                        };

                        let encoded: Vec<u8> = bincode::serialize(&updated).unwrap();
                        storage.set(sender, encoded);

                        let digest = last_event.map(|event| format!("{:x}", md5::compute(event)));
                        println!(
                            "{}",
                            style(format!(
                                "fetched last event from oracle {}, saved it to oracle's user state {:?}", 
                                oracle, digest)).cyan()
                        );
                    }
                }
            }
        }
    }
}
