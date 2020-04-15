
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use crossbeam::deque::{Injector,Stealer};
use chashmap::CHashMap;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;

use crate::vm::Config;
use crate::vm::Process;
use crate::vm::Message;
use crate::vm::Code;

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
    pub sleeping_counter: AtomicUsize
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

    pub fn add_process(&self, process: Process){
        self.injector.push(process);
        if self.sleeping_counter.load(Ordering::SeqCst) != 0 {
            self.wake_scheduler();
        }
    }

    pub fn wake_scheduler(&self){
        let (ref mutex, ref condition_variable) = self.thread_wait;
        let mut started = mutex.lock().unwrap();
        *started = true;
        condition_variable.notify_one();
    }
}