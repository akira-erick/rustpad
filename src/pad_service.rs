use crate::pad_persistence_trait;

pub struct PadService {
    persistence: Box<dyn pad_persistence_trait::PadPersistenceTrait>,
}

impl PadService {
    pub fn new(persistence: Box<dyn pad_persistence_trait::PadPersistenceTrait>) -> Self {
        PadService { persistence }
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        self.persistence.get(key)
    }

    pub fn set(&self, key: &str, value: String) -> Result<(), String> {
        self.persistence.set(key, value)
    }
}