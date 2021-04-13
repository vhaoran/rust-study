#[allow(unused_imports)]
#[allow(dead_code)]

#[test]
fn u_1() {
    struct Animal {
        age: i32,
    }

    struct Cat(Animal);
    struct Dog(Animal);
    struct LoudDog(Animal);

    trait Talk {
        fn talk(&self) -> ();
    }

    impl Talk for Cat {
        fn talk(&self) {
            println!("Meow");
        }
    }

    impl Talk for Dog {
        fn talk(&self) {
            println!("Woof!");
        }
    }

    impl Talk for LoudDog {
        fn talk(&self) {
            println!("WOOF!!");
        }
    }

    fn x() {
        let fluffy = Cat(Animal { age: 4 });
        let max = Dog(Animal { age: 2 });
        let neighbours_dog = LoudDog(Animal { age: 7 });

        fluffy.talk();
        max.talk();
        neighbours_dog.talk();
    }
    //-------------------------------------
    x();
}

#[test]
fn u_02() {
    fn escape_html(maybe_html: &str) -> String {
        let mut out = String::with_capacity(maybe_html.len());

        for c in maybe_html.chars() {
            match c {
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '&' => out.push_str("&amp;"),
                '\'' => out.push_str("&apos;"),
                '"' => out.push_str("&quot;"),
                _ => out.push(c),
            };
        }

        out
    }

    fn x() {
        let html = "<p>\"Hello, World!\"</p>";
        let escaped_html = escape_html(html);
        println!("{}", escaped_html);
    }
    //-------------------------------------
    x();
}


#[test]
fn u_04() {
    //---------------------
    use std::collections::{HashMap, HashSet};

    fn x() {
        let input_text = "does this work
  i dont know
  how rust works";

        let mut character_counts = HashMap::new();

        let mut n_lines = 0u32;

        for l in input_text.lines() {
            n_lines = n_lines + 1;

            let mut chars_for_line = HashSet::new();

            for c in l.chars() {
                if chars_for_line.contains(&c) {
                    continue;
                }
                let c_count = character_counts.entry(c).or_insert(0u32);
                *c_count += 1;
                chars_for_line.insert(c);
            }
        }

        for (c, c_count) in &character_counts {
            if *c_count == n_lines {
                println!("{}", c);
            }
        }
    }
    //-------------------------------------
    x();
}


#[test]
fn u_07() {
    //---------------------
    use std::str;

    #[derive(Debug)]
    struct User {
        id: u8,
        secret: String,
    }

    fn store_secrets(user: &User, buffer: &mut [u8]) {
        let _secret = user.secret.clone();


        // assume we're writing to a database
        println!("{:?}: {}", user, str::from_utf8(&buffer).unwrap());
    }

    fn x() {
        let buffer = &mut [0u8; 1024];
        let u1 = User {
            id: 1,
            secret: String::from("Pa55w0rd!"),
        };
        let u2 = User {
            id: 2,
            secret: String::from("correct horse battery staple"),
        };

        store_secrets(&u1, buffer);
        store_secrets(&u2, buffer);
    }
    //-------------------------------------
    x();
}


#[test]
fn word_count_0() {
    //---------------------
    use std::collections::HashMap;

    fn x() {
        let text = "once upon a time ...";
        let mut word_counts = HashMap::new();

        let pairs = text.split(" ")
            .map(|x| { (x, 1) });

        for (word, count) in pairs {
            let tmp = word_counts.entry(word)
                .or_insert(0);
            *tmp += count;
        }
        println!("{:?}", word_counts);
    }
    //-------------------------------------
    x();
}