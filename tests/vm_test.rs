#[cfg(test)]

mod tests {

    use clearvm::vm::Module;
    use clearvm::vm::VM;
    use std::collections::HashMap;

    #[test]
    fn create_vm() {
        let vm = VM::new(4,1024,1024);
        vm.code.set(1,Module::new(vec![3,2,1,3],HashMap::new(),0));
    }
}