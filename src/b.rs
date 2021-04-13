#[allow(unused_imports)]
#[allow(dead_code)]

extern crate url;
extern crate ws;
extern crate env_logger;
extern crate time;


// use std::time;
use ws::{Builder, CloseCode, Handler, Handshake, Message, Result, Sender, Settings};

const CONNECTIONS: usize = 3_000;
// simultaneous
const MESSAGES: u32 = 10;
static MESSAGE: &'static str = "ping";

pub fn test_b() {
    env_logger::init();



    struct Connection {
        out: Sender,
        count: u32,
        time: u64,
        total: u64,
    }

    impl Handler for Connection {
        fn on_open(&mut self, _: Handshake) -> Result<()> {
            self.out.send(MESSAGE)?;
            self.count += 1;
            self.time = time::OffsetDateTime::now_utc().timestamp() as u64;
            Ok(())
        }

        fn on_message(&mut self, msg: Message) -> Result<()> {
            //assert_eq!(msg.as_text().unwrap(), MESSAGE);
            println!("----b.rs--------{}" ,msg);
            if self.count > MESSAGES {
                self.out.close(CloseCode::Normal)?;
            } else {
                self.out.send(MESSAGE)?;
                let time = time::OffsetDateTime::now_utc().timestamp() as u64;
                self.total += time - self.time;
                self.count += 1;
                self.time = time;
            }
            Ok(())
        }
    }

    let mut ws = Builder::new()
        .with_settings(Settings {
            max_connections: CONNECTIONS,
            ..Settings::default()
        })
        .build(|out| Connection {
            out,
            count: 0,
            time: 0,
            total: 0,
        })
        .unwrap();

    for id in 0..CONNECTIONS {
        let s = format!("ws://0755yicai.com:8083/ws?jwt=test|{}", id);
        let u = url::Url::parse(s.as_str()).unwrap();
        ws.connect(u).unwrap();
        println!("----{}---", id);
    }
    let start = std::time::SystemTime::now();
    ws.run().unwrap();

    println!(
        "Total time. {}",
        std::time::SystemTime::now().duration_since(start).unwrap_or(
            std::time::Duration::from_secs(5)
        ).as_millis()
    )
}
