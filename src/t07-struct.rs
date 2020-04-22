fn test_struct() {
    {
        println!("-----------struct demo-----------");
        let bean = Employee {
            id: 1,
            name: String::from("no-name"),
        };
        println!("bean.id: {} ", bean.id);
        println!("bean.to_string(): {} ", bean.to_string());
    }
}

struct Employee {
    id: i64,
    name: String,
}

impl Employee {
    fn to_string(&self) -> String {
        let s = format!("{},{}", self.id, self.name);
        s.to_string()
    }
}

