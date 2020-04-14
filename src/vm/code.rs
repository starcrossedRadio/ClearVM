use std::sync::{Arc,RwLock};
use std::sync::atomic::{AtomicUsize};
use chashmap::CHashMap;

#[derive(Debug)]
pub struct Module{
    pub current: Arc<RwLock<Arc<Vec<u8>>>>
}

#[derive(Debug)]
pub struct Code{
    pub modules: CHashMap<u32,Module>
}

impl Code{

    pub fn new() -> Code {
        Code {
            modules: CHashMap::new()
        }
    }

    pub fn set(&self,hash: u32,code: Vec<u8>) {
        self.modules.insert(hash,Module { current: Arc::new(RwLock::new(Arc::new(code)))});
    }

    pub fn get(&self,hash: u32) -> Arc<Vec<u8>> {//-> Arc<Vec<u8>> {
        let module = self.modules.get(&hash).unwrap();
        let readable = (*module).current.read().unwrap();
        readable.clone()
    }
}