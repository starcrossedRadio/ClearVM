use chashmap::CHashMap;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub struct Module {
    pub code: Vec<u8>,
    pub jump_table: HashMap<u32,usize>,
    pub start_ip: usize
}

impl Module{
    pub fn new(code: Vec<u8>,jump_table: HashMap<u32,usize>,start_ip: usize) -> Module {
        Module {
            code,
            jump_table,
            start_ip
        }
    }
}

#[derive(Debug)]
pub struct Code {
    pub modules: CHashMap<u32, Arc<Module>>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            modules: CHashMap::new(),
        }
    }

    pub fn set(&self, hash: u32, module: Module) {
        self.modules.insert(
            hash,
            Arc::new(module),
        );
    }

    pub fn get(&self, hash: u32) -> Arc<Module> {
        let module = self.modules.get(&hash).unwrap();
        (*module).clone()
    }
}
