fn main() {
    println!("Hello, world!");
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4);
        assert_eq!(multiply(-1, 1), -1);
        assert_eq!(multiply(2, 0) , 0);
    }
}
