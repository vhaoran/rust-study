#[tokio::test]
async fn c_b_1() -> Result<(), Box<dyn std::error::Error>> {
    use crossbeam_channel::{select, unbounded};
    let (s, r) = unbounded();
    tokio::spawn(async move {
        for i in 0..1000 {
            let r = s.send(i);
            println!("-----------send result :{:?}-----------", r);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let data = r.try_recv();
            match data {
                Ok(i) => {
                    println!("-----------rec: {}-----------", i);
                }
                _ => {
                    println!("-----------not receive-----------");
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    })
    .await;

    tokio::spawn(async move {
        println!("----after sleep-----");
        for i in 0..10u32 {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----10 second sleep-----");
        }
    })
    .await;

    Ok(())
}
