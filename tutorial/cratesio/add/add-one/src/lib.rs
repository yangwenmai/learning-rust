use rand;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

pub fn add_one(x: i32) -> i32 {
    x+1
}
