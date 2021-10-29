use tokio::sync::mpsc::{Receiver, Sender};

#[tokio::test]
async fn mpsc_1() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(100);
    tokio::spawn(async move {
        loop {
            let v = rx.recv().await;
            match v {
                Some(data) => {
                    println!("-------rx:----{}-----------", data);
                    continue;
                }
                e @ _ => {
                    println!("-------rx none  {:?}", e);
                }
            }
            println!("-----------rx loop-----------");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    tokio::spawn(async move {
        for i in 0..100 {
            tx.send(i).await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    tokio::spawn(async move {
        for i in 0..10u32 {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----after sleep-----");
        }
    })
    .await;

    Ok(())
}
