#[cfg(test)]

mod tests {

    use clearvm::vm::Module;
    use clearvm::vm::Code;
    use std::collections::HashMap;  

    use std::sync::Arc;

    #[test]
    fn collect_old_version() {
        let code = Code::new();
        code.set(1,Module::new(vec![6,7,8,9],HashMap::new(),0));
        let old = code.get(1);
        code.set(1,Module::new(vec![3,4,5,6],HashMap::new(),0));
        let new = code.get(1);
        assert_eq!(Arc::new(Module::new(vec![6,7,8,9],HashMap::new(),0)),old);
        assert_eq!(Arc::new(Module::new(vec![3,4,5,6],HashMap::new(),0)),new);
    }
}