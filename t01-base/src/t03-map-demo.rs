use std::collections::HashMap;


fn map_demo() {
    {
        let mut m = HashMap::new();
        m.insert(String::from("a"), 10);
        m.insert(String::from("b"), 20);
        m.entry(String::from("c")).or_insert(30);

        for (k, v) in &m {
            println!("----------------------");
            println!(" key: {},value: {}", k, v);
            println!(" key: {},value: {}", k, v);
            println!(" key: {},value: {}", k, v);
        }
        println!("------map----------------");
        println!("{:?}", m);
    }
    {}
}