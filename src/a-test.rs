#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_abc() {
        abc!();
    }

    #[test]
    fn a() {
        abc!();
    }
}

fn abc() {
    println!("hello");
    prihtln!("0----------------------------");
}