//----------------------------------------
#[test]
fn count_test1() {
    count_1()
}

fn count_1() {
    println!("----------------------");
    let base = 1_000_000_000;
    let rate: f64 = 2.0 / 1.1;
    let year = 200;
    let i = count_n(base, year, rate);

    println!(" {} ->{} -> {} remain:--> {}",
             base, year, rate, i);
    println!("count: {}", i);
    println!("----------------------");

    fn count_n(base: i32, y: i32, rate: f64) -> f64 {
        let level = y / 25 as i32;
        println!("level: {}", level);
        //
        let r1 = 1.0 / rate.powi(level) * 1.0 * (base as f64);
        let r2 = 1.0 / rate.powi(level - 1) * (base as f64);
        let r3 = 1.0 / rate.powi(level - 2) * (base as f64);
        let r4 = 1.0 / rate.powi(level - 3) * (base as f64);
        //
        r1 + r2 + r3 + r4
    }
}
//----------------------------------------