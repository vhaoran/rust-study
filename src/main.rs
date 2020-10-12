// use std::ops::Add;

mod smartptr;
mod test1;
mod t01;
mod t02_str;
mod t03_map;
mod t04_vec;
mod t05_ownership;
mod t06_struct;
mod t07_sort;
mod t08_thread;
mod t09_struct;
mod tokio_demo;
mod t10_file;
mod t11_json;
mod t12_toml;
mod t13_async;
mod t14_log;
mod t15_yaml;

use std::thread::{park_timeout, sleep};

fn main() {
    use async_std::{
        task,
        //prelude::*,
        // Future或输入输出流
    };

    async fn f(uid: u64) {
        println!("-------------------------");
        extern crate ws;

        use ws::{Builder, Sender, Settings};

        let url = format!("ws://0755yicai.com:8083/ws?jwt=test|{}", uid);
        use ws::{connect, CloseCode};

        if connect(url, |out| {
            out.send("ping").unwrap();

            move |msg| {
                println!("-{}-----{}-",
                         std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
                         msg);
                task::sleep(std::time::Duration::new(60, 10_000));
                out.close(CloseCode::Normal)
            }
        }).is_err() {
            println!(" connection error")
        }
    }

    loop {
        for i in 0..900_000_000 {
            task::spawn(f(i as u64));
            std::thread::sleep(std::time::Duration::new(0, 1000_000));
        }
    }

    loop {
        println!("------------loop---{}--", std::time::UNIX_EPOCH.elapsed().unwrap().as_millis());
        std::thread::sleep(std::time::Duration::new(0, 1_000_000))
    }
}
