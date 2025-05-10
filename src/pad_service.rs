use crate::pad_persistence_trait;

pub struct PadService {
    persistence: Box<dyn pad_persistence_trait::PadPersistenceTrait>,
}

impl PadService {
    
}