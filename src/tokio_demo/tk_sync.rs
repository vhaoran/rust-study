#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
}
unsafe impl Send for User {}

//-----------a--------------------------
#[derive(Debug, Clone)]
pub struct R {
    pub data: String,
}
// unsafe impl Send for R {}

//-----------a--------------------------
#[derive(Debug)]
pub struct RErr(String);
impl std::error::Error for RErr {}

impl std::fmt::Display for RErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}", self.0);
        write!(f, "{}", s)
    }
}

//-----------a--------------------------
pub async fn f(i: i32) -> R {
    println!("-----------f() enter-----------");
    if i > 0 {
        R {
            data: " > 0".to_string(),
        }
    } else {
        R {
            data: " < 0".to_string(),
        }
    }
}

pub async fn f2() -> bool {
    true
}

#[tokio::test]
async fn tk_5() -> Result<(), Box<dyn std::error::Error>> {
    //-----------a--------------------------
    tokio::spawn(async move {
        let r = f(0).await;

        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        println!("-------------{:?}---------", r);
    })
    .await;
    Ok(())
}
#[tokio::test]
async fn tk_52() -> Result<(), Box<dyn std::error::Error>> {
    //-----------a--------------------------
    tokio::spawn(async move {
        let r = f2().await;
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        println!("-----------{:?}-----------", r);
    })
    .await;
    Ok(())
}
