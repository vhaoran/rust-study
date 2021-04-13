
#[test]
fn u_01() {
    //---------------------
    fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
        *i + *j
    }

    fn x() {
        let res = add_with_lifetimes(&10, &20);
        println!("{}", res);
    }
    //-------------------------------------
    x();
}

#[test]
fn u_02() {
    //---------------------
    fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
        *i + *j // <1>
    }

    fn x() {
        let res = add_with_lifetimes(&10, &20); // <2> <3>
        println!("{}", res);
    }
    //-------------------------------------
    x();
}

#[test]
fn u_04() {
    //---------------------
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    fn x() {
        let f = File::open("readme.md").unwrap();
        let reader = BufReader::new(f);

        for line_ in reader.lines() { // <1>
            let line = line_.unwrap(); // <2>
            println!("{} ({} bytes long)", line, line.len());
        }
    }
    //-------------------------------------
    let path = std::env::current_dir().unwrap();
    println!("#############The current directory is {}", path.display());


    x();
}