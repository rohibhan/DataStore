use std::collections::HashMap;
#[derive(Debug, Clone, Copy, Default)]
struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        Self.map.insert(key, value);
    }

    pub fn get(&self, key:String) -> Option<&String> {
        Self.map.get(&key)
    }

    pub fn remove(&mut self, key:String) {
        self.map.remove(&key);
    }
}
