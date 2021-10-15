use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn a_m_2() -> Result<(), Box<dyn std::error::Error>> {
    const HIGH: i32 = 1000;
    const TASK_HIGH: i32 = 10_000;
    let a: Arc<Mutex<HashMap<i32, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    //---set ------------
    let src = a.clone();
    tokio::spawn(async move {
        for k in 0..HIGH {
            for i in 0..1000 {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                {
                    let mut m = src.lock().await;
                    let s = format!("{}_{}_title", k, i);
                    if m.len() < 1000 {
                        if m.contains_key(&k) {
                            let mut l = m.get_mut(&k).unwrap();
                            l.push(s);
                        } else {
                            let mut l: Vec<String> = Vec::new();
                            l.push(s);
                            m.insert(k, l);
                        }
                    }
                }
            }
        }
    });

    //----get
    for i in 0..TASK_HIGH {
        let a = a.clone();
        tokio::spawn(async move {
            loop {
                for k in 0..HIGH {
                    tokio::time::sleep(std::time::Duration::from_millis(3000 * 10)).await;
                    {
                        let mut r = a.try_lock();
                        if r.is_err() {
                            continue;
                        }
                        let mut m = r.unwrap();
                        let mut r = m.get_mut(&k);
                        if let Some(l) = r {
                            if l.len() > 0 {
                                let s = l[0].clone();
                                println!(
                                    "-task: {}--get ({}: {})--of len {}---------------",
                                    i,
                                    k,
                                    s,
                                    l.len()
                                );
                                l.remove(0);
                            }
                        }
                    }
                }
            }
        });
    }

    tokio::spawn(async move {
        loop {
            println!("----loop in-----");
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    })
    .await;

    //
    Ok(())
}

#[tokio::test]
async fn a_m_1() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Clone)]
    pub struct Data {
        pub i: i32,
    }
    impl Data {
        pub fn add(&mut self, i: i32) {
            self.i += i
        }
    }

    //
    let a = Arc::new(Mutex::new(Data { i: 1 }));

    //
    let src = a.clone();
    tokio::spawn(async move {
        for i in 0..100 {
            {
                let a = src.clone();
                let mut v = a.lock().await;
                v.add(1);
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    Ok(())
}
