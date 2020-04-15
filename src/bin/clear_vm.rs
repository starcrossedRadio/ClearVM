use clearvm::vm::Module;
use clearvm::vm::Process;
use clearvm::vm::VM;
use std::collections::HashMap;
use std::{thread, time};

fn main() {
    let mut vm = VM::new(4, 1024, 1024);
    vm.state.code.set(1, Module::new(vec![0], HashMap::new(), 0));
    for i in 0..20 {
        let process = Process::new(vm.state.config.clone(), vm.state.code.get(1).clone(),i);
        vm.state.add_process(process);
    }
    vm.start();
    thread::sleep(time::Duration::from_secs(2));
}
