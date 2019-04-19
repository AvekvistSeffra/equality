use std::fmt;
use std::cmp::Ordering;
use std::ops::{ Add, Sub, Mul, Div, Rem, BitXor };
use serde_derive::{ Serialize, Deserialize };
use std::marker::Sized;

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
impl Number for f32 {}
impl Number for f64 {}
impl Number for Expr {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Val(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Val(x) => f64::from(*x),
            Expr::Add(x, y) => x.eval() + y.eval(),
            Expr::Sub(x, y) => x.eval() - y.eval(),
            Expr::Mul(x, y) => x.eval() * y.eval(),
            Expr::Div(x, y) => x.eval() / y.eval(),
            Expr::Rem(x, y) => x.eval() % y.eval(),
            Expr::Exp(x, y) => x.eval().powf(y.eval()),
        }
    }
}

impl From <i8> for Expr {
    fn from(value: i8) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<i16> for Expr {
    fn from(value: i16) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<i128> for Expr {
    fn from(value: i128) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From <u8> for Expr {
    fn from(value: u8) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<u16> for Expr {
    fn from(value: u16) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<u64> for Expr {
    fn from(value: u64) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<u128> for Expr {
    fn from(value: u128) -> Self {
        Expr::Val(i32::from(value))
    }
}

impl From<isize> for Expr {
    fn from(value: isize) -> Self {
        Expr::Val(value as i32)
    }
}

impl From<usize> for Expr {
    fn from(value: usize) -> Self {
        Expr::Val(value as i32)
    }
}

impl From<f32> for Expr {
    fn from(value: f32) -> Self {
        let denominator = 10_000_000;
        let numerator = (value * (denominator as f32)) as isize;

        let gcd = gcd(numerator, denominator);

        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        if denominator == 1 {
            Expr::from(numerator)
        } else {
            Expr::Div(
                Box::new(
                    Expr::from(numerator / gcd)
                ),
                Box::new(
                    Expr::from(denominator / gcd)
                )
            )
        }
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        let denominator = 10_000_000_000;
        let numerator = (value * (denominator as f64)) as isize;

        let gcd = gcd(numerator, denominator);

        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        if denominator == 1 {
            Expr::from(numerator)
        } else {
            Expr::Div(
                Box::new(
                    Expr::from(numerator / gcd)
                ),
                Box::new(
                    Expr::from(denominator / gcd)
                )
            )
        }
    }
}

impl<T> From<&T> for Expr 
    where T: Number {
    fn from(value: &T) -> Self {
        Expr::from(*value)
    }
}

impl<T> Add<T> for Expr
    where T: Number {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Expr::Add(
            Box::new(
                self
            ),
            Box::new(
                Expr::from(rhs)
            )
        )
    }
}

impl Add<Expr> for i8 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for i16 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for i32 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for i64 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for i128 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for u8 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for u16 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for u32 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for u64 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for u128 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for f32 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl Add<Expr> for f64 {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::from(self) + rhs
    }
}

impl<T> Sub<T> for Expr
    where T: Number {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Expr::Sub(
            Box::new(
                self
            ),
            Box::new(
                Expr::from(rhs)
            )
        )
    }
}

impl Sub<Expr> for i8 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for i16 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for i32 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for i64 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for i128 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for u8 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for u16 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for u32 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for u64 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for u128 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for f32 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl Sub<Expr> for f64 {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::from(self) - rhs
    }
}

impl<T> Mul<T> for Expr
    where T: Number {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = self {
            if x == 1 {
                return rhs;
            }
        }

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            }
        }

        Expr::Mul(
            Box::new(
                self
            ),
            Box::new(
                rhs
            )
        )
    }
}


impl Mul<Expr> for i8 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for i16 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for i32 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for i64 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for i128 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for u8 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for u16 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for u32 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for u64 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for u128 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for f32 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl Mul<Expr> for f64 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::from(self) * rhs
    }
}

impl<T> Div<T> for Expr
    where T: Number {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            } else {
                if let Expr::Val(y) = self {
                    let gcd = gcd(x as isize, y as isize);

                    return Expr::Div(
                        Box::new(
                            Expr::from(y / (gcd as i32))
                        ),
                        Box::new(
                            Expr::from(x / (gcd as i32))
                        )
                    );
                }
            }
        }

        Expr::Div(
            Box::new(
                self
            ),
            Box::new(
                rhs
            )
        )
    }
}

impl Div<Expr> for i8 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for i16 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for i32 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for i64 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for i128 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for u8 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for u16 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for u32 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for u64 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for u128 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for f32 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl Div<Expr> for f64 {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::from(self) / rhs
    }
}

impl<T> Rem<T> for Expr
    where T: Number {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Expr::Rem(
            Box::new(
                self
            ),
            Box::new(
                Expr::from(rhs)
            )
        )
    }
}

impl Rem<Expr> for i8 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for i16 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for i32 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for i64 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for i128 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for u8 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for u16 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for u32 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for u64 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for u128 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for f32 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl Rem<Expr> for f64 {
    type Output = Expr;

    fn rem(self, rhs: Expr) -> Self::Output {
        Expr::from(self) % rhs
    }
}

impl<T> BitXor<T> for Expr
    where T: Number {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output
        where precise::expression::Expr: std::convert::From<T> {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = self {
            if x == 0 {
                return Expr::from(x);
            }
        }

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            }
        }

        Expr::Exp(
            Box::new(
                self
            ),
            Box::new(
                rhs
            )
        )
    }
}

impl BitXor<Expr> for i8 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for i16 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for i32 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for i64 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for i128 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for u8 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for u16 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for u32 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for u64 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for u128 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for f32 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl BitXor<Expr> for f64 {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::from(self) ^ rhs
    }
}

impl<T> PartialEq<T> for Expr 
    where T: Number {
    fn eq(&self, rhs: &T) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl Eq for Expr {}

impl PartialOrd for Expr {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let lhs = self.eval();
        let rhs = rhs.eval();

        if lhs < rhs {
            Some(Ordering::Less)
        } else if lhs > rhs {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Expr {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = self.eval();
        let rhs = rhs.eval();

        if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Val(x) => write!(f, "{}", x),
            Expr::Add(x, y) => write!(f, "({} + {})", x, y),
            Expr::Sub(x, y) => write!(f, "({} - {})", x, y),
            Expr::Mul(x, y) => write!(f, "({} * {})", x, y),
            Expr::Div(x, y) => write!(f, "({} / {})", x, y),
            Expr::Rem(x, y) => write!(f, "({} % {})", x, y),
            Expr::Exp(x, y) => write!(f, "({} ^ {})", x, y),
        }
    }
}

pub fn gcd(numerator: isize, denominator: isize) -> isize {
    let mut a = numerator;
    let mut b = denominator;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn main() {
    let expr = 
        Expr::from(3) + 
        Expr::from(6) + 
        Expr::from(2) * 
        Expr::from(56) - 
        Expr::from(3) / 
        Expr::from(15) + 
        Expr::from(12);
    println!("{} = {}", expr, expr.eval());
}


#[cfg(test)]
mod tests {
    use super::Expr;

    #[test]
    fn from_i16() {
        let result = Expr::from(3_i16);
        let expected_result = Expr::Val(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Expr::from(3_i32);
        let expected_result = Expr::Val(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Expr::from(3.2_f32);
        let expected_result = Expr::Div(
            Box::new(
                Expr::from(16)
            ), Box::new(
                Expr::from(5)
            )
        );

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Expr::from(3.2_f64);
        let expected_result = Expr::Div(
            Box::new(
                Expr::from(16)
            ), Box::new(
                Expr::from(5)
            )
        );

        assert_eq!(result, expected_result);
    }

    #[test]
    fn evaluate() {
        let test_expression: Expr = (5 * (Expr::from(3) + 2 - Expr::from(0.5) / 5 * 2) / 31 + 55) / 23; 

        let result = test_expression.eval();
        let expected_result = ((5.0 * (5.0 - 1.0 / 5.0)) / 31.0 + 55.0) / 23.0;

        assert_eq!(result, expected_result);
/*
        let test_expression2: Expr = Expr::from(3) * (1 / (((Expr::from(3) ^ 2) + (Expr::from(4) ^ 2)) ^ 0.5));
        
        let result2 = test_expression2.eval();
        let expected_result2 = 0.6;

        assert_eq!(result2, expected_result2);

        let test_expression3: Expr = Expr::from(3) / (((Expr::from(3) ^ 2) + (Expr::from(4) ^ 2)) ^ 0.5);

        let result3 = test_expression3.eval();
        let expected_result3 = 0.6;

        assert_eq!(result3, expected_result3);
        */
    }

    #[test]
    fn add() {
        let lhs = Expr::from(5);
        let rhs = Expr::from(16);

        let result = lhs + rhs;
        let expected_result = 21;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn sub() {
        let lhs = Expr::from(5);
        let rhs = Expr::from(16);

        let result = lhs - rhs;
        let expected_result = -11;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn mul() {
        let lhs = Expr::from(5);
        let rhs = Expr::from(16);

        let result = lhs * rhs;
        let expected_result = 80;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn div() {
        let lhs = Expr::from(5);
        let rhs = Expr::from(16);

        let result = lhs / rhs;
        let expected_result = 5.0 / 16.0;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn rem() {
        let lhs = Expr::from(5);
        let rhs = Expr::from(16);

        let result = lhs % rhs;
        let expected_result = 5 % 16;

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn pow() {
        let result = Expr::from(5) ^ Expr::from(16);
        let expected_result = 5_f64.powf(16.0);

        assert_eq!(result, expected_result);
    }
}
