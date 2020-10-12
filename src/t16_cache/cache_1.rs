extern crate cached;


#[test]
fn cache_t_1() {
    use std::thread::sleep;
    use std::time::Duration;

    //extern crate cached;
    use cached::proc_macro::cached;
    use cached::SizedCache;


    /// Defines a function named `fib` that uses a cache explicitly named `FIB`.
    /// By default this will be the function in all caps.
    /// The following line is equivalent to #[cached(name = "FIB", unbound)]
    #[cached]
    fn fib(n: u64) -> u64 {
        println!("------------fib call-------------");
        if n == 0 || n == 1 { return n; }
        fib(n - 1) + fib(n - 2)
    }

    /// Use an lru cache with size 100 and a `(String, String)` cache key
    #[cached(size = 100)]
    fn keyed(a: String, b: String) -> usize {
        println!("------------keyed called-------------");
        let size = a.len() + b.len();
        //sleep(Duration::new(size as u64, 0));
        size
    }


    /// Use an explicit cache-type with a custom creation block and custom cache-key generating block
    #[cached(
    type = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}{}", a, b) }"#
    )]
    fn keyed2(a: &str, b: &str) -> usize {
        println!("------------keyed2 called-------------");
        let size = a.len() + b.len();
        // sleep(Duration::new(size as u64, 0));
        size
    }

    for i in 1..10 {
        println!("------------fib---------{}--{}--", i, fib(i % 3));
    }

    for i in 1..10 {
        let s1 = format!(" k-{}", i % 3);
        let s2 = format!(" v-{}", i + 1);
        println!("----#### keyed:-----{}----", keyed(s1, s2));
    }
    for i in 1..10 {
        let s1 = format!(" k-{}", i % 3);
        let s2 = format!(" v-{}", i % 3);
        println!("-******---keyed_2-{}----", keyed2(s1.as_str(), s2.as_str()));
    }
}