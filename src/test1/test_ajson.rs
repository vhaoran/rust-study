#[test]
fn asjon_1() {
    let data = r#"
   {
            "project": {
                "name": "ajson",
                "maintainer": "importcjj",
                "version": 0.1,
                "rusts": ["stable", "nightly"]
            }
   }
   "#;

    let name = ajson::get(data, "project.name").unwrap();
    println!("----------------------");
    println!("project.name {:?}", name);
    let r = ajson::get(data, "project").unwrap();
    println!("project {:?}", r);
}
