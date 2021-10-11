use chrono::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn arc_mutex_1() -> Result<(), Box<dyn std::error::Error>> {
    let a: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(Vec::new()));
    //
    for i in 0..10 {
        let aa = a.clone();
        tokio::spawn(async move {
            loop {
                {
                    let mut a = aa.lock().await;
                    let i = Local::now().timestamp();
                    a.push(i);
                    println!("----set-----");
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });
    }

    for i in 0..5 {
        let aa = a.clone();
        tokio::spawn(async move {
            loop {
                {
                    let mut a = aa.lock().await;
                    if a.len() > 0 {
                        let mut c = a[0].clone();
                        println!("-####  {}-len: {}-", c, a.len());
                        a.remove(0);
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });
    }

    tokio::spawn(async move {
        println!("----after sleep-----");
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----after sleep-----");
        }
    })
    .await;

    //
    Ok(())
}
