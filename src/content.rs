use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    content: String,
    key: String,
}

impl Content {
    #[allow(dead_code)]
    pub fn new(content: String, key: String) -> Self {
        Content { content, key }
    }
}
