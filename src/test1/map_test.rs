use std::collections::HashMap;
use std::string::ToString;

fn hashmap_test() {
    let mut contacts = HashMap::new();
    for i in 1..10 {
        contacts.insert(i, (i * 100).to_string());
    }
    contacts.insert(2, "222".to_string()); //overwrite <2, "200">
    contacts.remove(&3);

    contacts.entry(10).or_insert("7777".to_string()); // 如果contacts中没有key为10，则插入<10, 7777>；如果有key 10，则不插入，直接跳过；

    let len = contacts.len();
    println!("---------{}-------------", len);
    let if_contain_key = contacts.contains_key(&1);
    println!("---------{}-------------", if_contain_key);
    let value = contacts.get(&6);
    println!("---------{}-------------", value.unwrap());

    let keys = contacts.keys();
    println!("keys:{:?}", keys);
    let values = contacts.values();
    println!("values :{:?}", values);

    let value_key = contacts.get(&1).unwrap().clone();
    println!("value_key :{:?}", value_key);

    for (key, value) in contacts {
        println!("key {}, value {}", key, value);
    }

    let mut hash_vec: HashMap<u32, Vec<&str>> = HashMap::new();
    //对于HashMap<u32, Vec<&str>>类型，要对最里面Vec<&str>进行增加元素的操作（假设里面有“5”key），可以：
    hash_vec.insert(5, Vec::new());
    hash_vec.get_mut(&5).unwrap().push("value_5");

    let mut hash_map_tmp = HashMap::new();
    hash_map_tmp.insert(1, "value 1");//强制插入，已有key 1，则更新value为value 1；
    hash_map_tmp.entry(1).or_insert("value 2");//有key 1，不更新；没有key 1，插入 1， value 2

    //HashMap =>Vec
    let vec: Vec<(_, _)> = hash_map_tmp.into_iter().collect();
    println!("vec :{:?}", vec);

    let teams = vec![
        "AAA".to_string(),
        "BBB".to_string(),
        "CCC".to_string(),
    ];
    let socres = vec![10, 20, 30];
    let hash_map_scores: HashMap<_, _> = teams.iter().zip(socres.iter()).collect();
    println!("{:?}", hash_map_scores);


    {
        //统计单词个数
        let text = "aaa bbb ccc ddd aaa";
        let mut hmp = HashMap::new();
        let mut c = 0;
        for world in text.split_whitespace() {
            let count = hmp.entry(world).or_insert(0);
            c += 1;
        }
        println!("--------count: {}-------------", c);
    }
}

#[test]
fn hash_map_test_1() {
    hashmap_test();
}
