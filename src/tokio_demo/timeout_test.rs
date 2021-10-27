#[tokio::test]
async fn to_1() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        tokio::time::timeout(std::time::Duration::from_secs(7), async {
            for i in 0..10 {
                println!("----enter timeout---{}--", i);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        })
        .await;
        println!("----after timeout.await-----");
    });

    tokio::spawn(async move {
        for i in 0..2u32 {
            println!("----before sleep-----");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----after sleep-----");
        }
    })
    .await;

    Ok(())
}
