use std::cell::RefCell;

#[tokio::test]
async fn r_c_1() -> Result<(), Box<dyn std::error::Error>> {
    let c = RefCell::new(5);
    c.replace(20);
    let d = c.clone();
    d.replace(51);

    println!("------c-----{:?}-----------", c.clone());
    println!("------d-----{:?}-----------", d);

    Ok(())
}

pub async fn replace_spawn(c: &mut RefCell<i32>) {
    // wroing,l can't do it
    tokio::spawn(async move {
        for i in 0..100 {
            // c.replace(i);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
}
