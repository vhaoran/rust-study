use std::time::SystemTime;


fn vec_test() {
    {
        let l = [1, 2, 3];
        for each in l.iter() {
            println!("-------------{}---------", each);
        }
    }
    //-------------------------------------------
    {
        println!("---------begin time duration-------------");
        let t0 = SystemTime::now();

        let value = fib(40);
        println!("fir : {:?},{:?}", SystemTime::now().duration_since(t0).unwrap().as_secs(), value);
        println!("fir : {:?},{:?}", t0.elapsed().unwrap().as_secs(), value);
    }
    {
        println!("----------calc------------");
        let mut x = 0;
        for i in 1..3 {
            x = x + i;
        }
        println!(" x = {}", x);
        println!(" ------------calc end---------- ");
    }
    {
        println!("---------str add-------------");
        let l = ["aa", "bb", "cc", "dd", "ee", "ff"];
        let mut r = String::from("");
        for each in l.iter() {
            println!("each is : {}", each);
            let c = r.clone();
            r = r.add(c.as_ref()).add(each.as_ref());
        }
        println!(" str combine is : {}",r);
    }

    //-------------------------------------------
}

//-------------------------------------------

fn fib(x: i64) -> (i64, i64) {
    let a = &x;
    if x < 2 {
        return (x, 1);
    }

    let a1 = fib(x - 1);
    let a2 = fib(x - 2);

    (a1.0 + a2.0, a1.1 + a2.1 + 1)
}

