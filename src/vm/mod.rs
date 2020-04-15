pub mod code;
pub mod process;
pub mod scheduler;
pub mod state;
pub use code::Module;
pub use code::Code;
pub use process::Process;
pub use state::SharedState;
pub mod opcodes;
pub use opcodes::OpCode;

use std::sync::Arc;
use crossbeam::deque::Worker;
use scheduler::Scheduler;
use std::thread;


pub struct Message{
    pub code: Vec<u8>
}

// Initial configuraton of the VM
pub struct Config{
    pub scheduler_num: u16,
    pub heap_min_size: usize,
    pub stack_max_size: usize,
    pub cycles: usize,
}


// Module for holding some global properties between schedulers
pub struct VM {
    pub state: Arc<SharedState>,
}

impl VM{
    pub fn new(scheduler_num: u16,heap_min_size: usize,stack_max_size: usize) -> VM{
        VM {
            state: Arc::new(SharedState::new(scheduler_num,heap_min_size,stack_max_size)),
        }
    }

    pub fn create_scheduler(&self,id: usize) -> Scheduler{
        let local = Worker::<Process>::new_fifo();
        self.state.stealers.write().unwrap().push(local.stealer());
        Scheduler::new(self.state.clone(),local,id)
    }

    pub fn start(&mut self){
        for i in 0..self.state.config.scheduler_num{
            let scheduler = self.create_scheduler(i as usize);
            thread::spawn(move | | {
                scheduler.run();
            });
        }
    }
}