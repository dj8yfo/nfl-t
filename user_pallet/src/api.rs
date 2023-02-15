use engine::Account;

use crate::{OracleUser, State, PALLET_DESCRIPTOR};

impl OracleUser {
    pub fn state(account: Account, storage: &dyn engine::LedgerState) -> Option<State> {
        let key = (PALLET_DESCRIPTOR.to_string(), account);

        let raw_data = storage.get(key);

        raw_data.map(|raw_data| bincode::deserialize(&raw_data).unwrap())
    }
}
