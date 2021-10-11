use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::RwLock;

#[tokio::test]
async fn rw_lock_1() -> Result<(), Box<dyn std::error::Error>> {
    //r_base::init_module_n(None, true, false).await?;

    let a: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));

    for i in 0..3 {
        let b = a.clone();
        tokio::spawn(async move {
            let mut ok = false;

            while !ok {
                println!("----{}  {}---waiting --", i, ok);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                ok = {
                    let c = b.read().await;
                    *c
                };
            }
            println!("##@#####---{} end-----", i);
        });
    }

    //
    let x = a.clone();
    tokio::spawn(async move {
        let mut b = x.write().await;
        *b = true;
        println!("----**** set to true-----");
    })
    .await;

    tokio::spawn(async move {
        println!("----after sleep-----");
        let mut i = 0;
        loop {
            i += 1;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("--------{}-", i);
        }
    })
    .await;

    Ok(())
}
