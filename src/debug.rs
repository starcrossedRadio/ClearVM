#![macro_use]

macro_rules! debug_warn {
    ($($arg:tt)*) => { println!("[1m[33m[!][0m[33m {}[0m",format!($($arg)*)) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_warn {
    ($($arg:tt)*) => {}
}
/*
macro_rules! debug_normal {
    ($($arg:tt)*) => { println!("[1m[34m[!][0m[34m {}[0m",format!($($arg)*)) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_normal {
    ($($arg:tt)*) => {}
}*/

macro_rules! debug_ok {
    ($($arg:tt)*) => { println!("[1m[32m[+][0m[32m {}[0m",format!($($arg)*)) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_ok {
    ($($arg:tt)*) => {}
}

macro_rules! debug_fail {
    ($($arg:tt)*) => { println!("[1m[31m[-][0m[31m {}[0m",format!($($arg)*)) }
}

macro_rules! vm_panic {
    ($($arg:tt)*) => { println!("Panicked! {}",format!($($arg)*)); std::process::exit(1); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_fail {
    ($($arg:tt)*) => {}
}

/*
#[cfg(not(instruction))]
macro_rules! debug_instruction {
    ($($arg:tt)*) => {}
}



macro_rules! debug_stack {
    ($($arg:tt)*) => {}
}



macro_rules! debug_reg {
    ($($arg:tt)*) => {}
}

macro_rules! debug_title {
    ($($arg:tt)*) => { println!("\x1b[34m───── {} ─────[0m\x1b",format!($($arg)*)) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_title {
    ($x:expr) => {}
}
*/