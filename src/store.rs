use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// The main, persistent key-value store.
/// It's wrapped in Arc<Mutex<>> to be shared safely across threads.
#[derive(Debug, Clone)]
pub struct KeyValueStore {
    /// The main data storage.
    data: Arc<Mutex<HashMap<String, String>>>,
    /// A set of keys that are currently locked by active transactions.
    locked_keys: Arc<Mutex<HashSet<String>>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            locked_keys: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn del(&self, key: &str) {
        let mut data = self.data.lock().unwrap();
        data.remove(key);
    }

    /// Attempts to lock a set of keys. Returns true if successful.
    pub fn lock_keys(&self, keys: &HashSet<String>) -> bool {
        let mut locked = self.locked_keys.lock().unwrap();
        if keys.iter().any(|k| locked.contains(k)) {
            return false; // One of the keys is already locked
        }
        for key in keys {
            locked.insert(key.clone());
        }
        true
    }

    /// Unlocks a set of keys.
    pub fn unlock_keys(&self, keys: &HashSet<String>) {
        let mut locked = self.locked_keys.lock().unwrap();
        for key in keys {
            locked.remove(key);
        }
    }
}
