use engine::{Account, RawTx};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TxPayload {
    Initialize { oracle_account: Account },
    Watch,
}

impl super::OracleUser {
    pub fn init_tx(from: Account, oracle: Account) -> RawTx {
        let payload = TxPayload::Initialize {
            oracle_account: oracle,
        };
        let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();
        RawTx {
            from: (super::PALLET_DESCRIPTOR.to_string(), from),
            payload: encoded,
        }
    }

    pub fn watch_tx(from: Account, _useful_payload: Vec<u8>) -> RawTx {
        let payload = TxPayload::Watch;
        let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();
        RawTx {
            from: (super::PALLET_DESCRIPTOR.to_string(), from),
            payload: encoded,
        }
    }
}
