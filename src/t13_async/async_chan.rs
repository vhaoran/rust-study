#[allow(unused_imports)]
#[allow(dead_code)]
use std::sync::mpsc::SyncSender;

#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn async_chan_1() {
    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };
    #[allow(unused_imports)]
    #[allow(dead_code)]
    use std::sync::mpsc::{channel, sync_channel};
    use std::thread;
    use std::time;

    let (tx, rx) = sync_channel::<String>(0);
    async fn test3(tx: SyncSender<String>) -> i32 {
        println!("-----test3 exec-->async exec---------------");

        let s = format!("--test3---{}----{}--", task::current().id(),
                        time::UNIX_EPOCH.elapsed().unwrap().as_secs());
        let _r = tx.send(s);
        1 + 2
    }

    let high = 30;
    for _i in 1..=high {
        let _h = task::spawn(test3(tx.clone()));
        println!("------------After-------------");
    }

    for i in rx.iter() {
        println!("------------{}-------------", i);
    }
}


// impl Cnt {}
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
#[allow(dead_code)]
pub fn global_data() -> &'static Arc<Mutex<HashMap<i64, String>>> {
    static INSTANCE: OnceCell<Arc<Mutex<HashMap<i64, String>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(1, "Spica".to_string());
        m.insert(2, "Hoyten".to_string());
        m.insert(3, "Hoyten".to_string());
        Arc::new(Mutex::new(m))
    })
}


#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn async_once_cell_1() {
    #[allow(unused_imports)]
    #[allow(dead_code)]

    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };

    #[allow(unused_imports)]
    #[allow(dead_code)]
    use std::sync::mpsc::{channel, sync_channel};
    use std::thread;
    use std::time;

    let (tx, rx) = sync_channel::<String>(0);
    #[allow(unused_imports)]
    #[allow(dead_code)]
    async fn test3(tx: SyncSender<String>) -> i32 {
        println!("-----test3 exec-->async exec---------------");

        let s = format!("--test3---{}----{}--", task::current().id(),
                        time::UNIX_EPOCH.elapsed().unwrap().as_secs());
        let _r = tx.send(s.clone());

        let x = Arc::clone(global_data());
        let mut m = x.lock().unwrap();

        let id = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        thread::sleep(time::Duration::new(1, 0));
        for i in id..id + 100 {
            m.insert(i as i64, s.clone());
        }

        1 + 2
    }

    let high = 30;
    for _i in 1..=high {
        let _h = task::spawn(test3(tx.clone()));
        println!("------------After-------------");
    }


    for i in rx.iter() {
        println!("------------{}-------------", i);
        let x = Arc::clone(global_data());
        let m = x.lock().unwrap();
        println!("------------{}-------------", m.len());
    }
}

