pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&self, key: String, value: String) -> Option<String> {
        unimplemented!();
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!();
    }

    pub fn remove(&self, key: String) -> Option<String> {
        unimplemented!();
    }
}
