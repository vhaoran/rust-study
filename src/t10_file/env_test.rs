#[test]
fn test_workdir() {
    use std::env;

    fn get() -> Option<String> {
        let path = env::current_dir().unwrap();
        let a = 3;



        println!("The current directory is {}", path.display());
        Some(path.to_str().unwrap().to_string())
    }

    let r = get();

    if let Some(s) = r {
        println!("  ok ,get workdir,  {} ----------", s)
    }
}

#[test]
fn a_t(){


}
