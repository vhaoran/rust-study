use std::time::Duration;

use tokio::task;

#[tokio::test]
async fn test_tk_task1() {
    let _a = task::spawn(async {
        println!("----tk_02.rs--------");
    })
    .await;

    println!("-----------");
    std::thread::sleep(Duration::from_secs(3));
}

#[tokio::test]
async fn tk_2() -> Result<(), Box<dyn std::error::Error>> {
    pub async fn f() -> bool {
        println!("-----------f() enter-----------");
        true
    }
    tokio::spawn(async move {
        if f().await {
            f().await;
        }
        println!("-----------ok----------");
    })
    .await;
    Ok(())
}

#[tokio::test]
async fn tk_3() -> Result<(), Box<dyn std::error::Error>> {
    pub async fn f() -> String {
        println!("-----------f() enter-----------");
        "f()".to_string()
    }
    tokio::spawn(async move {
        if f().await.len() > 0 {
            let r = f().await;
            println!(" r : {:?}", r);
        }
        println!("-----------ok----------");
    })
    .await;
    Ok(())
}

#[tokio::test]
async fn tk_4() -> Result<(), Box<dyn std::error::Error>> {
    pub async fn f() -> Result<String, Box<dyn std::error::Error>> {
        println!("-----------f() enter-----------");
        Ok("result<String,box<(>dyn>> ".to_string())
    }
    tokio::spawn(async move {
        if f().await.is_ok() {
            let r = f().await.unwrap();
            println!(" r : {:?}", r);
        }
        if f().await.is_err() {
            let r = f().await.unwrap();
            println!(" r : {:?}", r);
        }

        println!("-----------ok----------");
    })
    .await;
    Ok(())
}

#[tokio::test]
async fn tk_5() -> Result<(), Box<dyn std::error::Error>> {
    pub async fn f() -> Result<String, Box<dyn std::error::Error>> {
        println!("-----------f() enter-----------");
        Ok("result<String,box<(>dyn>> ".to_string())
    }
    tokio::spawn(async move {
        let mut ok = false;
        {
            let r = f().await;
            ok = match r {
                Err(e) => false,
                _ => true,
            }
        }

        if ok {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            println!("-----------sleep-----------");
        } else {
            println!("-----------ok----------");
        }
    })
    .await;
    Ok(())
}

#[tokio::test]
async fn tk_6() -> Result<(), Box<dyn std::error::Error>> {
    pub async fn f() -> Result<String, Box<dyn std::error::Error>> {
        println!("-----------f() enter-----------");
        Ok("result<String,box<(>dyn>> ".to_string())
    }
    tokio::spawn(async move {
        let mut s = "".to_string();
        let mut ok = false;
        {
            let r = f().await;
            match r {
                Err(e) => {
                    s = e.to_string();
                    ok = false;
                }
                Ok(str) => {
                    s = str;
                    ok = true;
                }
            }
        }

        if ok {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            println!("-----------sleep--s: {}--------", s);
        } else {
            println!("-----------ok----------");
        }
    })
    .await;
    Ok(())
}
