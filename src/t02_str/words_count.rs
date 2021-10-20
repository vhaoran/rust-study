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
