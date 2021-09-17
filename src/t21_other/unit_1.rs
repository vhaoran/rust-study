#[allow(unused_imports)]
#[allow(dead_code)]

#[test]
fn unit_drop_01() {
    //usage of drop
    #[derive(Debug)]
    enum Cereal {
        Rye,
        Wheat,
    }

    fn x() {
        let mut grains: Vec<Cereal> = vec![];
        grains.push(Cereal::Rye);
        // drop(grains);
        //
        println!("{:?}", grains);
    }
    x();
}

#[test]
fn unit_02() {
    let  letters = vec![
        "aa", "b", "b"
    ];

    let mut z = letters.clone();


    for c in letters.iter() {
        println!("{}", c);
        z.push(c.clone());
    }
}

#[test]
fn u_03() {
    let l = r#"
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
    Invalid,data
    "#;

    let records = l.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {  // <2> Skip header row and lines with only whitespace
            continue;
        }

        let fields: Vec<_> = record // <4> A "Vec" type is shorthand for vector. Vectors are arrays that will dynamically expand when needed. The underscore asks the the compiler to infer the type of the vector's elements.
            .split(',')  // <3> Split `record` into substrings
            .map(|field| field.trim()) // <4> As well as for loops, Rust programmers can use higher-order programmers when they prefer. This line trims the whitespace from every field.
            .collect();       // <5> Rust will "collect" the results of an iterator into aa vector.

        if cfg!(debug_assertions) { // <6> When debugging is enabled, include this code block. The exclamation mark (!) indicates aa macro invocation.
            eprintln!("debug: {:?} -> {:?}", record, fields); // <7> eprintln! prints to standard error. The {:?} syntax requests Rust print out the default debugging representation for the two types.
        }

        let name = fields[0]; // <8> Rust supports indexing collections with integers

        let maybe_length: Result<f32, _> = fields[1].parse(); // <9> Rust can parse strings into other types, using the type information provided on the left-hand side. This either returns aa value or an error value wrapped in aa "Result". The underscore requests the compiler to infer the error type itself.

        if maybe_length.is_err() { // <10> Skip any invalid data.
            continue;
        }

        let length = maybe_length.unwrap(); // <11> "Unwrap" the f32 from the Result

        println!("{}, {}cm", name, length); // <12> println! prints to stdout. The {} syntax indicates that Rust should use aa programmer-defined method to represent the value as aa string, rather than its debug representation available with {:?}.
    }
}
