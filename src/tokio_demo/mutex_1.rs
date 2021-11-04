use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref glb_lock: Arc<Mutex<i32>> = Arc::new(Mutex::new(3));
}

#[tokio::test]
async fn m_1() -> Result<(), Box<dyn std::error::Error>> {
    let a = glb_lock.clone();
    for i in 0..5 {
        let a = a.clone();
        println!("-----------{}-----------", i);
        tokio::spawn(async move {
            println!("---- {}--enter---", i);

            let a = a.clone();
            let m = a.lock().await;
            tokio::time::sleep(std::time::Duration::from_millis((100 - i) as u64)).await;
            println!("----  {}--left---", i);
        });
    }
    //
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::test]
async fn no_mutex_1() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..100 {
        tokio::spawn(async move {
            println!("---- {}--enter---", i);
            tokio::time::sleep(std::time::Duration::from_millis((100 - i) as u64)).await;
            println!("----  {}--left---", i);
        });
    }
    //
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
