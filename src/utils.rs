pub trait Number {}

pub fn greatest_common_divisor(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::greatest_common_divisor;

    #[test]
    fn gcd() {
        let val1 = 250;
        let val2 = 20;

        let result = greatest_common_divisor(val1, val2);
        let expected_result = 10;

        assert_eq!(result, expected_result);
    }
}
