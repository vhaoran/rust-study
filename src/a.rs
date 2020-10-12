extern crate async_std;
extern crate env_logger;
extern crate ws;
extern crate log;


fn a() {
    use async_std::{
        task,
        prelude::*,
        // Future或输入输出流
    };

    async fn f(uid: u64) {
        println!("-------------------------");
        extern crate ws;

        use ws::{Sender, Settings};

        let url = format!("ws://0755yicai.com:8083/ws?jwt=test|{}", uid);
        use ws::{connect, CloseCode};

        if connect(url, |out| {
            out.send("ping").unwrap();

            move |msg| {
                println!("-{}-----{}-",
                         std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
                         msg);
                //
                // task::block_on(task::sleep(Duration::from_secs(60)));
                for i in 0..10000 {
                    println!("ww");
                }
                out.close(CloseCode::Normal);
                Ok(())
            }
        }).is_err() {
            println!(" ********************connection error ****************")
        }
    }

    loop {
        for i in 0..10000 {
            task::spawn(f(i as u64));
            std::thread::sleep(std::time::Duration::new(0, 1_000_000));
        }
    }
}
