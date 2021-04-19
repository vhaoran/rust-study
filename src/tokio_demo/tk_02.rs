use std::time::Duration;

use tokio::task;

#[tokio::test]
async fn test_tk_task1() {
    let _a = task::spawn(async{
        println!("----tk_02.rs--------");
    }).await;

    println!("-----------");
    std::thread::sleep(Duration::from_secs(3));
}


