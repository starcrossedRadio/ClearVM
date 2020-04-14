use crate::vm::Module;
use crate::vm::Config;
use std::sync::Arc;

#[derive(Debug)]
struct StackFrame{
    pub ip: usize,
    pub module: Arc<Module>,
    pub registers: Vec<u32>
}

#[derive(Debug)]
pub struct Process{
    memory: Vec<u8>,
    registers: [u32;16],
    ip: usize,
    sf: Vec<StackFrame>,
    module: Arc<Module>,
    pub halted: bool,
    pub blocked: bool,
}

impl Process{
    pub fn new(config: Arc<Config>,module: Arc<Module>) -> Process{
        Process {
            memory: vec![0;config.heap_min_size + config.stack_max_size],
            registers: [0;16],
            ip: module.start_ip,
            module,
            sf: Vec::new(),
            blocked: false,
            halted: false,

        }
    }

    pub fn execute_instruction(&mut self) {

    }
}