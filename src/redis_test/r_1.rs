


#[test]
fn r_1() {
    use redisclient::RedisClient;
    let cfg = redisclient::config::RedisConfig{
        address:"192.168.0.99:6379".to_string(),
        database: 0 ,
        username: None,
        password: None,
        pool_capacity: 10,
    };

    let mut c1 = RedisClient::with_config(cfg).unwrap();
    let mut c = c1.expire()

    let k = "aaaa";
    let v = "v_value_aa";
    c.set(k.to_string(),v.to_string(),Some(1024) , None , None ,None);

    let r:String = c.get(k.to_string()).unwrap_or("".to_string());
    println!("----r_1.rs---a--{:#?}" ,r);
}
