pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn divide(left: u64, right: u64) -> Option<u64> {
    if right == 0 {
        return None;
    }
    Some(left / right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtract_works() {
        let result = subtract(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn multiply_works() {
        let result = multiply(3, 4);
        assert_eq!(result, 12);
    }

    #[test]
    fn divide_works() {
        assert_eq!(divide(12, 4), Some(3));
        assert_eq!(divide(10, 0), None);
    }
}
