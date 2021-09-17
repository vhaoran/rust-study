use std::sync;
use std::sync::atomic::{AtomicBool, Ordering};
use sync::Arc;

#[test]
fn a_t_c_1() {
    use std::sync::Barrier;
    //---------------------
    let a = AtomicBool::new(false);
    a.store(true, Ordering::Relaxed);
    println!("------true:----{}-----------", a.load(Ordering::Relaxed));
    a.store(false, Ordering::Relaxed);
    println!("---flase:-------{}-----------", a.load(Ordering::Relaxed));
}

#[tokio::test]
async fn a_a_1() -> Result<(), Box<dyn std::error::Error>> {
    let c: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let c1 = c.clone();
    tokio::spawn(async move {
        for i in 0..50 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            c1.store(i % 2 == 1, Ordering::Relaxed);
            println!("-----------set value-----------");
        }
    });

    let c2 = c.clone();
    tokio::spawn(async move {
        for i in 0..10 {
            println!("-----####------{}-----------", c2.load(Ordering::Relaxed));
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
    .await;

    println!("-----------end tasks-----------");

    Ok(())
}
