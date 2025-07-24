use crate::command::Command;
use crate::store::KeyValueStore;
use std::collections::{HashMap, HashSet};

// A temporary write log for a transaction. `Option` allows for deletions.
type WriteSet = HashMap<String, Option<String>>;

pub struct Transaction {
    writeset: WriteSet,
    active: bool,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            writeset: HashMap::new(),
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn begin(&mut self) -> Result<(), &'static str> {
        if self.active {
            return Err("Transaction already in progress");
        }
        self.active = true;
        self.writeset.clear();
        Ok(())
    }

    /// Adds a SET or DEL operation to the transaction's writeset.
    pub fn stage_write(&mut self, command: Command) -> Result<(), &'static str> {
        if !self.active {
            return Err("No active transaction");
        }
        match command {
            Command::Set(key, value) => {
                self.writeset.insert(key, Some(value));
            }
            Command::Del(key) => {
                self.writeset.insert(key, None);
            }
            _ => return Err("Invalid command for a transaction"),
        }
        Ok(())
    }

    /// Gets a value, checking the transaction's writeset first, then the main store.
    pub fn get_value(&self, key: &str, store: &KeyValueStore) -> Option<String> {
        if let Some(value_opt) = self.writeset.get(key) {
            return value_opt.clone(); // Return staged write/delete
        }
        store.get(key) // Fallback to main store
    }

    /// Commits the transaction to the main store.
    pub fn commit(&mut self, store: &KeyValueStore) -> Result<(), &'static str> {
        if !self.active {
            return Err("No active transaction");
        }

        let keys_to_lock: HashSet<String> = self.writeset.keys().cloned().collect();
        if !store.lock_keys(&keys_to_lock) {
            self.rollback(store); // Abort on lock failure
            return Err("Conflict detected, transaction rolled back");
        }

        // This is where a real distributed system would send the writeset
        // to the Raft consensus module. For now, we apply it directly.
        for (key, value_opt) in self.writeset.drain() {
            match value_opt {
                Some(value) => store.set(key, value),
                None => store.del(&key),
            }
        }

        store.unlock_keys(&keys_to_lock);
        self.active = false;
        Ok(())
    }

    /// Rolls back the transaction, discarding all changes.
    pub fn rollback(&mut self, store: &KeyValueStore) {
        let keys_to_unlock: HashSet<String> = self.writeset.keys().cloned().collect();
        store.unlock_keys(&keys_to_unlock); // Ensure any held locks are released
        self.writeset.clear();
        self.active = false;
    }
}
