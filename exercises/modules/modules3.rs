// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.



// TODO: Complete this use statement
use std::time::{SystemTime, Duration};  
  
fn main() {  
    // 获取 Unix 时间戳的 SystemTime 表示  
    let unix_epoch = SystemTime::UNIX_EPOCH;  
  
    match SystemTime::now().duration_since(unix_epoch) {  
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),  
        Err(_) => panic!("Current time is before UNIX EPOCH! This should not happen."),  
    }  
}