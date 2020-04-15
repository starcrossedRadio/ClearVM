use crate::vm::Module;
use crate::vm::Config;
use crate::vm::OpCode;
use std::sync::Arc;
use std::convert::TryInto;

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
    pub id: usize
}

impl Process{
    pub fn new(config: Arc<Config>,module: Arc<Module>,id: usize) -> Process{
        Process {
            memory: vec![0;config.heap_min_size + config.stack_max_size],
            registers: [0;16],
            ip: module.start_ip,
            module,
            sf: Vec::new(),
            blocked: false,
            halted: false,
            id
        }
    }

    pub fn get_next_byte(&mut self) -> u8{
        if self.module.code.len() < self.ip+1 { vm_panic!("Invalid Operand!"); }
        if self.module.code[self.ip] >= 16 { vm_panic!("Invalid Register!"); }
        self.ip += 1;
        self.module.code[self.ip-1]
    }

    pub fn get_4_bytes(&mut self) -> u32{
        if self.module.code.len() < self.ip+4 { vm_panic!("Invalid Operand!"); }
        self.ip += 4;
        let (int_bytes, _) = self.module.code[self.ip-4..self.ip].split_at(std::mem::size_of::<u32>());
        u32::from_le_bytes(int_bytes.try_into().unwrap())
    }

    pub fn execute_instruction(&mut self) {
        match OpCode::from(self.get_next_byte()) {
            OpCode::Halt => {
                self.halted = true;
            }
            _ => {
                unimplemented!();
            }
        }
    }
}