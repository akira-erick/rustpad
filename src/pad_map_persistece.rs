use crate::pad_persistence_trait::PadPersistenceTrait;
use std::collections::HashMap;

pub struct PadMapPersistence {
    map: HashMap<String, String>,
}

#[allow(dead_code)]
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
