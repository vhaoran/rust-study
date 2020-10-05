#[test]
fn test_workdir() {
    use std::env;

    fn get() -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());
        Ok(())
    }

    let r = get();
    if let Ok(()) = r {
        println!("  ok ,get workdir")
    }
}
