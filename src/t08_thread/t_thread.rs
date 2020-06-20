#[test]
fn thread_test1() {
    thread_1();
}

use std::sync::{Mutex, Arc};
use std::thread;

#[allow(dead_code)]
fn thread_1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for zid in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _k in 0..5 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!(" do num + 1 of  #### {} ####", zid)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("---------result is -------------");
    println!("Result: {}", *counter.lock().unwrap());
}
