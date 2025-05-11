use crate::pad_map_persistece;
use crate::pad_persistence_trait;

#[allow(dead_code)]
pub struct PadService {
    persistence: Box<dyn pad_persistence_trait::PadPersistenceTrait>,
}

#[allow(dead_code)]
impl PadService {
    pub fn new() -> Self {
        PadService {
            persistence: Box::new(pad_map_persistece::PadMapPersistence::new()),
        }
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        self.persistence.get(key)
    }

    pub fn set(&mut self, key: &str, value: String) -> Result<(), String> {
        self.persistence.set(key, value)
    }
}
