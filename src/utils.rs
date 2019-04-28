use std::ops::{ Add, Sub, Mul, Div };
use std::marker::Sized;
use crate::precise::expression::Expr;

pub trait Number: Add + Sub + Mul + Div + Sized {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for isize {}
impl Number for usize {}
impl Number for f32 {}
impl Number for f64 {}
impl Number for Expr {}

pub fn gcd(numerator: i128, denominator: i128) -> i128 {
    let mut a = numerator;
    let mut b = denominator;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd() {
        let val1 = 250;
        let val2 = 20;

        let result = super::gcd(val1, val2);
        let expected_result = 10;

        assert_eq!(result, expected_result);
    }
}
