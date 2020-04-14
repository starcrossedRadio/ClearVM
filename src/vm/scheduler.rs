
use std::sync::Arc;
use crate::vm::SharedState;
use crate::vm::Process;
use std::sync::atomic::Ordering;

use crossbeam::deque::{Worker,Steal};

pub struct Scheduler {
    pub state: Arc<SharedState>,
    pub processes: Worker<Process>,
    pub id: usize,
}

impl Scheduler{
    pub fn new(state: Arc<SharedState>,processes: Worker<Process>,id: usize) -> Scheduler{
        Scheduler{
            state,
            processes,
            id
        }
    }

    fn execute_process(&self,mut process: Process) {
        for _ in 0..200 {
            process.execute_instruction();
        }
    }

    fn block(&self) {
        self.state.sleeping_counter.fetch_add(1, Ordering::SeqCst);
        let (ref mutex, ref condition_variable) = self.state.thread_wait;
        let mut started = mutex.lock().unwrap();
        *started = false;
        debug_warn!("Thread {} Slept n:{}",self.id,self.state.sleeping_counter.load(Ordering::SeqCst));
        while !*started {
            started = condition_variable.wait(started).unwrap();
        }
        self.state.sleeping_counter.fetch_sub(1, Ordering::SeqCst);
        debug_ok!("Thread {} woke up",self.id);
        //println!("{}",self.state.sleeping_counter.load(Ordering::SeqCst));
    }
    
    fn steal(&self) {
        if let Steal::Success(_) = self.state.injector.steal_batch(&self.processes) {
            
        }else{
            self.block();
        }
    }

    pub fn run(&self) {
        loop {
            while let Some(process) = self.processes.pop() {
                self.execute_process(process);
            }    
            if self.processes.is_empty() {
                self.steal();
            }
        }
    }
}