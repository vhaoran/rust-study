#[test]
fn words_max_len_1() {
    let l = "a hello good test aa aa aa".to_string();
    let (mut old, mut wc, mut j) = (0, 0, 0);
    for c in l.chars() {
        if c == ' ' {
            wc = j;
            j = 0;
            if old < wc {
                old = wc;
            }
        } else {
            j += 1;
        }
    }
    if old < wc {
        old = wc;
    }

    println!("-----------max: {}-----------", old);
}

#[test]
fn f_1() {
    //---------------------
    let r = std::fs::read_dir("./").unwrap();
    for v in r {
        println!("--------------{:?}--------", v);
        //
        let p = v.unwrap();
        println!("-----------file_name:  {:?}--------", p.file_name().clone().into_string());
    }

    if std::path::Path::new("./data").exists(){
        println!("-----------exists-----------");
    }else{
        println!("---------not--exists-----------");
    }

    if std::path::Path::new("./src").exists(){
        println!("-----------exists-----------");
    }else{
        println!("---------not--exists-----------");
    }

}


fn queue_name() -> String {
    let s = std::env::args().nth(0).unwrap();
    s
}

#[test]
fn app_path_1() {
    //---------------------
    println!("-----------{}-----------",queue_name());
}
