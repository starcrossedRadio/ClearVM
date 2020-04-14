
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

    fn execute_process(&self,process: &mut Process) {
        for _ in 0..self.state.config.cycles {
            process.execute_instruction();
        }
    }

    fn block(&self) {
        self.state.sleeping_counter.fetch_add(1, Ordering::SeqCst);
        debug_warn!("Thread {} Slept n:{}",self.id,self.state.sleeping_counter.load(Ordering::SeqCst));
        if self.state.sleeping_counter.load(Ordering::SeqCst) == self.state.config.scheduler_num as usize {
            std::process::exit(0);
        }
        let (ref mutex, ref condition_variable) = self.state.thread_wait;
        let mut started = mutex.lock().unwrap();
        *started = false;
        while !*started {
            started = condition_variable.wait(started).unwrap();
        }
        self.state.sleeping_counter.fetch_sub(1, Ordering::SeqCst);
        debug_ok!("Thread {} woke up",self.id);
    }
    
    fn steal(&self) {
        if let Steal::Success(_) = self.state.injector.steal_batch(&self.processes) {
            debug_ok!("Thread {} stole from injector",self.id);
        }else{
            let stealers = self.state.stealers.read().unwrap();
            for stealer in stealers.iter() {
                if let Steal::Success(_) = stealer.steal_batch(&self.processes) {
                    debug_ok!("Thread {} stole from another thread!",self.id);
                    break;
                }
            }
            if self.processes.is_empty() {
                drop(stealers);
                self.block();
            }
        }
    }

    pub fn run(&self) {
        loop {
            while let Some(mut process) = self.processes.pop() {
                self.execute_process(&mut process);
                if !process.halted {
                    self.processes.push(process);
                }else{
                    debug_fail!("Process {} halted in Scheduler {}!",process.id,self.id);
                }
            }    
            if self.processes.is_empty() {
                self.steal();
            }
        }
    }
}