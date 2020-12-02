extern crate serde_yaml;

#[test]
fn yaml_2_fn() {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64,
    }

    fn x() -> Result<(), serde_yaml::Error> {
        let point = Point { x: 1.0, y: 2.0 };

        let s = serde_yaml::to_string(&point)?;
        println!("----yaml_2.rs---{}-----", s);
        assert_eq!(s, "---\nx: 1.0\ny: 2.0");

        let pt: Point = serde_yaml::from_str(&s)?;
        assert_eq!(point, pt);
        println!(" {:#?} ", pt);

        Ok(())
    }

    let _ = x();
}