#[test]
fn err_1(){
   println!("----abc-----");

   let path = "./a.txt".to_string();

   let i = file_len(path).unwrap_or(0);
   println!("----err_test_1.rs---a  len is {}-----" ,i);
}



fn file_len(p :String)->Result<usize,String>{
   let s = std::fs::read_to_string(p.as_str()).unwrap_or("".to_string());
   Ok(s.len())
}

#[test]
fn err_map_1() {
   let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
   let path = "./readme.md".to_string();

   let i = file_len_of_map(path);
   println!("----err_test_1.rs---a  len is {:#?}-----" ,i);
}

fn file_len_of_map(p :String)->Result<usize,String>{
   std::fs::read_to_string(p.as_str())
       .map_err(|e|e.to_string())
       .and_then(|c|Ok(c.len()))
}
