#[allow(unused_imports)]
#[allow(dead_code)]
use std::sync::mpsc::sync_channel;

#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn channel_1() {
    #[allow(unused_imports)]
    #[allow(dead_code)]

    use std::thread;
    // use std::sync::mpsc::sync_channel;
    use std::time;
    use std::thread::sleep;

    let (tx, rx) = sync_channel::<String>(0);
    let tx2 = tx.clone();
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100 {
            let s = format!("data: {}", i);
            tx.send(s).unwrap();
            sleep(time::Duration::new(1, 0));
        }
    });

    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100 {
            let s = format!("data 2: {}", i);
            tx2.send(s).unwrap();
            sleep(time::Duration::new(1, 0));
        }
    });

    for i in rx.iter() {
        println!("---------rec:---{}----", i);
    }
}

