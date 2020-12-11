extern crate async_std;

use self::async_std::task::JoinHandle;
use std::ops::Add;


#[test]
fn async_a() {
    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };

    let a = task::block_on(async {
        println!("-------------------------");
        println!("Hello, world!");
        "----async result----".to_string()
    });

    println!("----async_a.rs---result is :  {}", a);
}


#[test]
fn async_2() {
    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };

    async fn test2() {
        println!("-----test2 exec-----async exec---------------");
    }

    task::block_on(test2())
}


#[test]
fn async_3() {
    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };
    use std::thread;
    use std::time;

    async fn test3() -> i32 {
        println!("-----test3 exec-->async exec---------------");
        // let id = unsafe {
        //     std::thread::current().id().as_u64();
        // };

        println!("-----test3 inner------taskID: {}----{}--", task::current().id(),
                 std::thread::current().name().unwrap_or("none"));
        1 + 2
    }

    let high = 2;
    for _i in 1..=high {
        let _h = task::spawn(test3());
        println!("------------After-------------");
    }
    //let z = h.await;
    //h.task()
    // println!("z = {}", z);
    println!("------------before sleep-------------");
    let sec = time::Duration::from_secs(2);
    thread::sleep(sec);
}

#[test]
fn async_t_01() {
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };
    async fn x(i: u32) -> String {
        println!("----async_a.rs-id:  {} --i: {} ---", task::current().id(), i);
        task::sleep(std::time::Duration::from_secs(1)).await;

        "xxx".to_string()
    }

    for i in 0..10 {
        task::spawn(x(i));
    }

    println!("--##############################-----------");
    std::thread::sleep(std::time::Duration::from_secs(2));
}

#[test]
fn async_t_02() {
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };
    async fn x(i: u32) -> String {
        println!("-----async_a.rs--i:  {} -------", i);
        println!("----async_a.rs-id:  {} -----", task::current().id());
        "xxx".to_string()
    }

    // let b = x(100).await;
    let z = task::spawn(x(100));
    // task::spawn_blocking(x(200));
    async fn y(h: JoinHandle<String>) -> String {
        let r = h.await;
        r.add("---good")
    }

    let y = task::block_on(y(z));
    println!("----async_a.rs--y: {}-----", y);

    println!("--##############################-----------");
}

