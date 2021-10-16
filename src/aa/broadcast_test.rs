use tokio::sync;

use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

#[tokio::test]
async fn b_c_1() -> Result<(), Box<dyn std::error::Error>> {
    //
    const HIGH: i32 = 2;
    let (tx, mut rx): (Sender<i32>, Receiver<i32>) = broadcast::channel(10);

    for i in 0..2 {
        let sender = tx.clone();
        tokio::spawn(async move {
            loop {
                for i in 0..10000_i32 {
                    sender.send(i).map_err(|e| {
                        println!("--send--error---{}-", e.to_string());
                    });
                    // println!(
                    //     "---------receiver count:--{}-----------",
                    //     sender.receiver_count()
                    // );
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                }
            }
        });
    }

    // tx.send(10).unwrap();

    // The receiver lagged behind
    // assert!(rx.recv().await.is_err());
    for i in 0..HIGH {
        let mut rx = tx.clone().subscribe();
        // let mut rx = rx.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(v) = rx.recv().await {
                    println!("--#####-- {} -> {}---", i, v);
                }
                // tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                // println!("----{} wait loop-----", i,);
            }
        });
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        println!("-----------sleep-----------");
    }

    Ok(())
}
