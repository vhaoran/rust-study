#[allow(dead_code)]
#[test]
fn thread_1() {
    use std::thread;
    //    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

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


#[allow(dead_code)]
#[test]
fn thread_2() {
    //use std::collections::HashMap;
    //use std::string::String;
//    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;

    let m: HashMap<String, String> = HashMap::new();

    let c = Arc::new(Mutex::new(m));
    let mut handles = vec![];

    println!("-------------------------");

    for zid in 0..10 {
        let x = Arc::clone(&c);
        let handle = thread::spawn(move || {
            for k in 0..500000 {
                let mut z = x.lock().unwrap();
                z.insert(k.to_string(), "hello world".to_string());

                println!(" do num + 1 of  #### {} ####", zid)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("---------result is -------------");
    println!("Result: {}", c.lock().unwrap().len());
}
