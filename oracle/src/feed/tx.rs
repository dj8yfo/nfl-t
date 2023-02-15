use engine::{Account, RawTx};

use super::Event;

impl super::Feed {
    pub fn data_tx(external_data: Event, account: Account) -> RawTx {
        RawTx {
            from: (super::PALLET_DESCRIPTOR.to_string(), account),
            payload: external_data,
        }
    }
}
