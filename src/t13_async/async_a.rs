extern crate async_std;


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

    task::block_on(async {
        println!("-------------------------");
        println!("Hello, world!");
    })
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
