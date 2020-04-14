pub mod code;
pub mod process;
pub use code::Module;
pub use code::Code;
pub use process::Process;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use crossbeam::deque::Injector;

// Initial configuraton of the VM
pub struct Config{
    pub scheduler_num: u16,
    pub heap_min_size: usize,
    pub stack_max_size: usize,
}

// Module for holding some global properties between Schedulers
pub struct VM {
    pub code: Arc<Code>,
    pub config: Arc<Config>,
    pub injector: Injector<Process>,
    sleeping_counter: AtomicUsize
}

impl VM{
    pub fn new(scheduler_num: u16,heap_min_size: usize,stack_max_size: usize) -> VM{
        VM {
            config: Arc::new(Config {
                scheduler_num,
                heap_min_size,
                stack_max_size
            }),
            code: Arc::new(Code::new()),
            injector: Injector::new(),
            sleeping_counter: AtomicUsize::new(0)
        }
    }

    pub fn start(&self){

    }
}