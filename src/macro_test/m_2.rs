#[test]
fn m_2_1() {
    let result = (1..5).fold(0, |acc, x| acc + x * x);
    println!("result = {:#?}", result);
    let numbers = [1, 2, 3, 4, 5];
    let zero = "0".to_string();
    let result = numbers
        .iter()
        .fold(zero, |acc, &x| format!("({} + {})", acc, x));

    println!("-----------{:#?}-----------", result);
    let numbers = [1, 2, 3, 4, 5];
    let zero = "".to_string();
    let result = numbers.iter().fold(zero, |acc, &x| {
        if acc.len() == 0 {
            format!("{}", x)
        } else {
            format!("{},{}", acc, x)
        }
    });

    println!("-----------{:#?}-----------", result);
}
