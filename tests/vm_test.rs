#[cfg(test)]

mod tests {

    use clearvm::vm::Module;
    use clearvm::vm::Process;
    use clearvm::vm::VM;
    use std::collections::HashMap;

    #[test]
    fn create_vm() {
        let mut vm = VM::new(4, 1024, 1024);
        vm.state.code.set(1, Module::new(vec![3, 2, 1, 3], HashMap::new(), 0));
        for i in 0..20 {
            let process = Process::new(vm.state.config.clone(), vm.state.code.get(1).clone(),i);
            vm.state.injector.push(process);
        }
        vm.start();
    }
}