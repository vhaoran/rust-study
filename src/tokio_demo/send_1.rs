use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
struct Data {
    inner: Arc<Mutex<i32>>,
    i: i32,
}
impl Data {
    pub fn new() -> Self {
        Data {
            inner: Arc::new(Mutex::new(0)),
            i: 5,
        }
    }
    pub async fn set(&mut self, i: i32) {
        println!("----before lock of {}-----", i);

        let mut a = self.inner.lock().await;
        *a = i;
        println!("----after lock of {}-----", i);
    }
    pub async fn set_i(&mut self, i: i32) {
        let a = self.inner.lock().await;
        // let mut a = self.inner.lock().await;
        // *a = i;
        self.i = i + 1;
    }

    pub async fn get(&self) -> (i32, i32) {
        let a = self.inner.lock().await;
        (a.clone(), self.i)
    }
}

#[tokio::test]
async fn send_1() -> Result<(), Box<dyn std::error::Error>> {
    let a = Data::new();
    for k in 0..20 {
        let mut b = a.clone();
        let i = k;
        tokio::spawn(async move {
            b.set(i).await;
            b.set_i(i).await;
        });
    }

    //----------b-----------
    let b = Data::new();
    for k in 0..4 {
        let mut b = b.clone();
        let i = k;
        tokio::spawn(async move {
            b.set(i).await;
            b.set_i(i).await;
        });
    }
    //

    for i in 0..10 {
        println!(
            "-----------a: {:?}---b: {:?}--------",
            a.get().await,
            b.get().await
        );
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
