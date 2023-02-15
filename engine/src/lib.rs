pub mod simple_ledger;
pub type Account = String;
pub type PalletDescriptor = String;
pub trait LedgerState {
    fn set(&mut self, key: (PalletDescriptor, Account), value: Vec<u8>);
    fn get(&self, key: (PalletDescriptor, Account)) -> Option<Vec<u8>>;
}

pub struct RawTx {
    pub from: (PalletDescriptor, Account),
    pub payload: Vec<u8>,
}

pub trait Ledger {
    fn process_tx(&mut self, tx: RawTx);
}

pub trait Pallet {
    fn process_tx(&mut self, tx: &RawTx, storage: &mut dyn LedgerState);
}
