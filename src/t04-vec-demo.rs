#[test]
fn t04() {
    vec_test();
}


use std::time::SystemTime;

fn vec_test() {
    {
        let l = [1, 2, 3];
        for each in l.iter() {
            println!("-------------{}---------", each);
        }

        //----------------------------------------
        for each in l.iter().rev() {
            println!("-------------{}---------", each);
        }
        //----------------------------------------
    }
    //-------------------------------------------
    {
        println!("---------begin time duration-------------");
        let t0 = SystemTime::now();

        let value = fib(10);
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
        println!(" str combine is : {}", r);
    }
    {
        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("vector: {:?}", v);
    }
    {
        //2: vec!宏使用初始值来创建一个 Vec
        let v = vec![1, 2, 3];// v 离开作用域并被丢弃, 然后新建一个vector名字也叫做v
        println!("demo2-vector: {:?}", v)
    }
    {
        let mut v = vec![0; 10];
        println!("{:?}", v);
        //[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        v[0] = 1;
        v[1] = 2;
        println!("{:?}", v[0]);
        println!("{:?}", v);
        //可以使用索引访问vector中的值
    }
    {
        // &v[index]    返回一个引用，
        // v.get(index) 方法返回一个 Option < &T >。
        // 如果越界访问的话，&v[index]会panic，
        // v.get(index)会返回None，
        // 所以如果要引用vector的元素，建议使用v.get(index)
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        let g_third: Option<&i32> = v.get(2);
        println!("{}, {:?}", third, g_third);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        // let third:&i32 = &v[100]; //thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
        let g_third: Option<&i32> = v.get(100);
        println!("{:?}", g_third);
        println!("{:?}", g_third == None);
    }
    {
        // 最好使用引用来循环遍历元素。引用获取值得使用权而不是控制权
        let mut v = vec![1, 2, 3, 4, 5];

        for i in &mut v {
            *i += 100;
            //为了修改可变引用所指向的值，
            // 在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
        }

        for i in &v {
            println!("{}", i)
        }

        let mut arr = [1, 2, 3];
        for i in &mut arr {
            *i += 100;
        }
        for i in &arr {
            println!("{}", i)
        }
    }
    {
        // 如果不使用元素的引用来遍历元素。直接遍历的话，vector将会失去那一片内存的控制权
        let v = vec![1, 2, 3, 4, 5];

        for i in v {   //v将失去这块内存得所有权
            println!("{}", i)
        }

        // will panic
        // for i in v {  //error[E0382]: use of moved value: `v`
        //     println!("{}", i)
        // }
    }
    {
        // 存储不同类型的元素：vector结合enum
        #[derive(Debug)]
        enum XCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            XCell::Int(3),
            XCell::Text(String::from("blue")),
            XCell::Float(11.12),
        ];
        println!("{:?}", row);
    }
    {
        assert_eq!(2, 2);
    }
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            // Prints 3, 2, 1
            println!("{}", top);
        }
    }

//-------------------------------------------
}

//-------------------------------------------
#[test]
fn test_fib() {
    let (v, all) = fib(20);
    println!("a: {} b:{}", v, all);
}

fn fib(x: i64) -> (i64, i64) {
    let a = &x;
    println!("{}", a);
    if x < 2 {
        return (x, 1);
    }

    let a1 = fib(x - 1);
    let a2 = fib(x - 2);


    let v = (a1.0 + a2.0, a1.1 + a2.1 + 1);
    println!("###  x: {} ->  {} ###", &v.0, &v.1);
    v
}

