pub mod code;
pub mod process;
pub mod scheduler;
pub use code::Module;
pub use code::Code;
pub use process::Process;

use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use crossbeam::deque::{Injector,Worker,Stealer};
use chashmap::CHashMap;
use scheduler::Scheduler;
use std::thread;
use std::collections::VecDeque;


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

// The data that will be share between multiple schedulers.
pub struct SharedState{
    pub code: Arc<Code>,
    pub config: Arc<Config>,
    pub current_id: AtomicUsize,
    pub stealers: RwLock<Vec<Stealer<Process>>>,
    pub thread_wait: (Mutex<bool>, Condvar),
    pub message_buffer: CHashMap<usize,VecDeque<Message>>,
    pub waiting: CHashMap<usize,Process>,
    pub injector: Injector<Process>,
    sleeping_counter: AtomicUsize
}

impl SharedState{
    pub fn new(scheduler_num: u16,heap_min_size: usize,stack_max_size: usize) -> SharedState{
        SharedState {
            config: Arc::new(Config {
                scheduler_num,
                heap_min_size,
                stack_max_size,
                cycles: 200
            }),
            code: Arc::new(Code::new()),
            current_id: AtomicUsize::new(0),
            stealers: RwLock::new(Vec::new()),
            thread_wait: (Mutex::new(false), Condvar::new()),   
            message_buffer: CHashMap::new(),
            waiting: CHashMap::new(),
            injector: Injector::new(),
            sleeping_counter: AtomicUsize::new(0)
        }
    }
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
        for i in 0..(self.state.config.scheduler_num-1){
            let scheduler = self.create_scheduler(i as usize);
            thread::spawn(move | | {
                scheduler.run();
            });
        }
        let scheduler = self.create_scheduler((self.state.config.scheduler_num-1) as usize);
        scheduler.run();
    }
}