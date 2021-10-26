use std::time::Duration;

#[tokio::test]
async fn f_1() -> std::result::Result<(), Box<dyn std::error::Error>> {
    //---------------------
    let (tx, rx) = flume::unbounded();

    tokio::spawn(async move {
        for i in 0..20 {
            tx.send(i).unwrap();
            println!("-----------aftger send------len:-{}----",tx.len());
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    // Spawn tasks

    tokio::spawn(async move {
        loop {
            match rx.try_recv() {
                Ok(i) => {
                    println!("-#####--receive: {}--len: {}--", i,rx.len());
                }
                _ => {
                    println!("-*****--receive: error---");
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    tokio::spawn(async move {
        println!("----after sleep-----");
        for i in 0..10u32 {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            println!("----loop-----");
        }
    })
    .await;

    Ok(())
}
