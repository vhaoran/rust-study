#[test]
fn t07() {
    test_struct();
}

fn test_struct() {
    {
        println!("-----------struct demo-----------");
        let age = 10;
        let bean = Employee {
            id: 1,
            name: String::from("no-name"),
            age,
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
        };
        let  bean1 = Employee {
            id: 2,
            ..bean
        };
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

#[derive(Debug)]
struct Employee {
    id: i64,
    name: String,
    age: i64,
}


impl Employee {
    fn to_string(&self) -> String {
        let s = format!("id: {},name: {},age: {}", self.id, self.name, self.age);
        s.to_string()
    }
}

