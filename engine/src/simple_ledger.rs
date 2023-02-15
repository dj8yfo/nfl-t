use std::collections::HashMap;

use crate::{Account, Ledger, LedgerState, Pallet, PalletDescriptor, RawTx};

pub type Storage = HashMap<(PalletDescriptor, Account), Vec<u8>>;

pub struct MemoryLedger {
    log: Vec<RawTx>,
    pallets: HashMap<PalletDescriptor, Box<dyn Pallet + Send>>,

    storage: Storage,
}

impl MemoryLedger {
    pub fn new(pallets: HashMap<PalletDescriptor, Box<dyn Pallet + Send>>) -> Self {
        Self {
            log: vec![],
            pallets,
            storage: HashMap::new(),
        }
    }
}

impl LedgerState for Storage {
    fn set(&mut self, key: (PalletDescriptor, Account), value: Vec<u8>) {
        self.insert(key, value);
    }

    fn get(&self, key: (PalletDescriptor, Account)) -> Option<Vec<u8>> {
        self.get(&key).cloned()
    }
}

impl Ledger for MemoryLedger {
    fn process_tx(&mut self, tx: RawTx) {
        if let Some(pallet) = self.pallets.get_mut(&tx.from.0) {
            pallet.process_tx(&tx, &mut self.storage);
        }
        self.log.push(tx);
    }
}
