#[cfg(test)]

mod tests {

    use clearvm::vm::Module;
    use clearvm::vm::Code;

    use std::sync::Arc;
    use std::sync::RwLock;
    use std::{thread, time};

    #[test]
    fn collect_old_version() {
        let mut code = Code::new();
        code.set(1,vec![6,7,8,9]);
        let old = code.get(1);
        code.set(1,vec![3,4,5,6]);
        let new = code.get(1);
        assert_eq!(Arc::new(vec![6,7,8,9]),old);
        assert_eq!(Arc::new(vec![3,4,5,6]),new);
    }
}