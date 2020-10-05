extern crate async_std;

#[test]
fn sync_a() {
    //    use futures::executor::block_on;
    use async_std::{
        // fs::File,
        // 支持异步操作的文件结构体
        task,
        // 调用调度器
        //prelude::*,
        // Future或输入输出流
    };

    task::spawn()
    task::block_on(async {
        println!("-------------------------");
        println!("Hello, world!");
    })
}