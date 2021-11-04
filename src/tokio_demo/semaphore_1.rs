use tokio::sync::Semaphore;

#[tokio::test]
async fn s_1() -> Result<(), Box<dyn std::error::Error>> {
    //
    let sem = Semaphore::new(2);
    let r = sem.acquire().await?;
    //-------------------------------------
    println!("----------acquire r: {:?}-----------", r);
    println!(
        "----------available_permits: {}-----------",
        sem.available_permits()
    );

    //-------------------------------------
    {
        let r = sem.acquire().await?;
        println!("----------acquire r: {:?}-----------", r);
        drop(r);
    }

    println!(
        "----------available_permits: {}-----------",
        sem.available_permits()
    );

    //-------------------------------------
    println!("--waiting-------");
    let r = sem.acquire().await?;
    println!("----------acquire r: {:?}-----------", r);
    println!(
        "----------available_permits: {}-----------",
        sem.available_permits()
    );

    println!(
        "----------available_permits:-{}-----------",
        sem.available_permits()
    );

    Ok(())
}
