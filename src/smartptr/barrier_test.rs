#[tokio::test]
async fn t_b_1() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;
    use tokio::sync::Barrier;

    let barrier = Arc::new(Barrier::new(10));
    // c.wait().await;
    // let c = Arc::clone(&barrier);

    // Spawn tasks
    for id in 0..9 {
        println!("----id {}---", id);
        let c = Arc::clone(&barrier);
        tokio::spawn(async move {
            println!("{}  before wait", id);
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            c.wait().await;
            println!("{} after wait", id);
        });
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let c = Arc::clone(&barrier);
    tokio::spawn(async move {
        println!(" last ***** before wait");
        c.wait().await;
        println!("last *** after wait");
    });

    tokio::spawn(async move {
        println!("----after sleep-----");
        for i in 0..5u32 {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            println!("----leep 10-----");
        }
    })
    .await;

    Ok(())
}

#[tokio::test]
async fn t_b_1_no_barrier() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn tasks
    for id in 0..9 {
        println!("----id {}---", id);
        tokio::spawn(async move {
            println!("{}  before wait", id);
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            println!("{} after wait", id);
        });
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    tokio::spawn(async move {
        println!(" last ***** before wait");
        println!("last *** after wait");
    });

    tokio::spawn(async move {
        println!("----after sleep-----");
        for i in 0..10u32 {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            println!("----leep 10-----");
        }
    })
    .await;

    Ok(())
}
