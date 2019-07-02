pub struct KvStore {
}

pub enum Values { Error1, Error2 }


impl KvStore {
    pub fn new() -> KvStore {
        KvStore { }
    }

    pub fn set(&self, key: String, value: String) -> Option<String> {
        Some("set".to_string())
    }

    pub fn get(&self, key: String) -> Option<String> {
        Some("get".to_string())
    }

    pub fn remove(&self, key: String) -> Option<String> {
        Some("remove".to_string())
    }
}
