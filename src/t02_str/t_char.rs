#[test]
fn c_1() {
    //---------------------
    let s = "中国人民解放军";
    for i in s.chars() {
        println!("-----------a------{}-----", i);
        println!("-----------a------{:X}-----", i as u32);
    }
}

#[test]
fn c_2() {
    //---------------------
    let c = '>';
    for i in c as u32..(c as u32 + 10) {
        let cc = char::from_u32(i).unwrap_or('n');
        println!("-----------a------{}-----", cc);
        println!("--------{:X}-----", cc as u32);
    }
}

#[test]
fn c_3() {
    //-----------2----------
    let c = '❤';
    let end = 0x2894u32;
    let start = end - 5000;

    println!("--------{:X} -- {:X}-----------", start, end);

    for i in start..end {
        let cc = char::from_u32(i).unwrap_or(' ');
        if cc == ' ' {
            break;
        }
        print!("{}", cc);
    }
    println!("----------------------");
}
