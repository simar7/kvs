#[derive(Clone, Copy, Default)]
pub struct KvStore {

}

impl KvStore {
    pub fn new() -> Self {
        KvStore{}
    }

    pub fn set(self, key1: String, key2: String) {
        panic!()
    }

    pub fn get(self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(self, key: String) {
        panic!()
    }
}