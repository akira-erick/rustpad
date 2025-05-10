pub trait PadPersistenceTrait {
    fn get(&self, key: &str) -> Result<String, String>;
    fn set(&self, key: &str, value: String) -> Result<(), String>;
}