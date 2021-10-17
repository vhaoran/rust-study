#[test]
fn t07() {
    test_struct();
}

#[allow(dead_code)]
fn test_struct() {
    {
        println!("-----------struct demo-----------");
        let age = 10;
        let bean = Employee {
            id: 1,
            name: String::from("no-name"),
            age,
            list: Vec::new(),
        };
        println!("bean.id: {} ", bean.id);
        println!("bean.to_string(): {} ", bean.to_string());
        println!("bean.to_string(): {:?} ", bean);
    }
    {
        println!("-----------struct demo 2-----------");
        let age = 10;
        let bean = Employee {
            id: 1,
            name: String::from("no-name"),
            age,
            list: Vec::new(),
        };
        let bean1 = Employee { id: 2, ..bean };
        println!("bean1.id: {} ", bean1.id);
        println!("bean1.to_string(): {} ", bean1.to_string());
    }
    {
        println!("-----tuple struct-----------------");

        struct Color(i32, i32, String);
        let c = Color(1, 2, String::from("color struct"));
        println!("color: ({},{},{})", c.0, c.1, c.2);
        println!("color: ({},{},{})", c.0, c.1, c.2);
    }
}

#[derive(Debug, Default)]
struct Employee {
    id: i64,
    name: String,
    age: i64,
    list: Vec<i64>,
}

impl Employee {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let s = format!("id: {},name: {},age: {}", self.id, self.name, self.age);
        s
    }
}

#[test]
fn struct_a_1() {
    //---------------------
    let src = Employee::default();
    println!("-----------{:#?}-----------", src);
    println!("-----------{}-----------", src.list.len());
}
