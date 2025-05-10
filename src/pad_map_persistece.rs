use std::collections::HashMap;
use crate::pad_persistence_trait::PadPersistenceTrait;

pub struct PadMapPersistence {
    map: HashMap<String, String>,
}

impl PadMapPersistence {
    pub fn new() -> Self {
        PadMapPersistence {
            map: HashMap::new(),
        }
    }
}

impl PadPersistenceTrait for PadMapPersistence {
    fn get(&self, key: &str) -> Result<String, String> {
        match self.map.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Key not found: {}", key)),
        }
    }
    fn set(&mut self, key: &str, value: String) -> Result<(), String> {
        self.map.insert(key.into(), value);
        Ok(())
    }
}
