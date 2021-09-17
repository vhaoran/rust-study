extern crate redis_rs;

#[test]
fn red_2() {
    use redis_rs::connection::Connection;
    use redis_rs::response::RedisResponse;
    use std::net::TcpStream;

    let host = "192.168.0.99";
    let port = 6379;
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).unwrap();

    // stream can be anything that implements read and write
    let mut client = Connection::new(host, port, stream);

    // send aa request
    let _ = client.send_raw_request("SET FOO1 BAR");
    // or use aa supported command
    client.set("whr", "whr_value");

    let r = client.get("FOO1").unwrap();

    println!("----r_2.rs---{:?}-----", r);
}
