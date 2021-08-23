#[test]
fn for_test() {
    do_for();
}

#[allow(dead_code)]
fn do_for() {
    println!("--------demo for  -----------");
    {
        for i in 0..5 {
            println!("i = {} ", i);
        }
    }
    //----------------------------------------
    {
        let a = [10, 20, 30, 40, 50];
        for i in a.iter() {
            println!("值为 : {}", i);
        }
    }
    {
        let a = [10, 20, 30, 40, 50];
        for i in a.iter().rev() {
            println!("值为 : {}", i);
        }
    }
    {
        for i in (0..21).filter(|x| (x % 2 == 0)) {
            print!("{} ", i);
        }
    }
    {
        for i in (1..11).map(|x| x * x) {
            print!("{} ", i);
        }
    }
    {
        // fold()是一个非常强大的方法。
        // 它返回一个特殊的“累加器”类型闭包的结果给迭代器的所有元素，
        // 得到一个单一的值。 下面的迭代器产生从1到5的数字的平方和。
        let result = (1..5).fold(0, |acc, x| acc + x * x);
        println!("result = {}", result);
    }
    {
        //需要一个不连续的Range
        // （基本上是两个不相Range的组合）？
        // 您可以使用chain()方法组合多个范围：
        let c = (1..4).chain(6..9);
        for i in c {
            print!("{} ", i);
        }
    }
    {
        let cities = ["Toronto", "New York", "Melbourne"];
        let populations = [2_615_060, 8_550_405, 4_529_500];

        let matrix = cities.iter().zip(populations.iter());

        for (c, p) in matrix {
            println!("{:10}: population = {}", c, p);
        }
    }
}

#[test]
fn for_test1() {
    for_result();
}

#[allow(dead_code)]
fn for_result() {
    let mut k = 0;
    let a = loop {
        k += 1;
        if k == -3 {
            break k + 10000;
        }
        if k == 3 {
            break 20;
        }
    };

    println!("a = for result is : {}", a);
    println!("a = for result is : {}", a);
    println!("a = for result is : {}", a);
}

#[test]
fn for_test2() {
    for_result2();
}

#[allow(dead_code)]
fn for_result2() {
    let a = {
        let mut b = 0;
        for i in 0..101 {
            b += i;
        }
        b
    };

    println!("a = for result is : {}", a);
    println!("a = for result is : {}", a);
    println!("a = for result is : {}", a);
}

#[test]
fn div_floor_1() {
    //---------------------
    let i = 60i32;
    let c = i / 7;
    println!("----------------------");

    println!(" {:?}", c);
}
