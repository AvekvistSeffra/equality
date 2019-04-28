use std::fmt;
use std::cmp::Ordering;
use std::ops::{ Add, Sub, Mul, Div, Rem, BitXor };
use serde_derive::{ Serialize, Deserialize };
use crate::utils::gcd;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Val(i128),
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
            Expr::Val(x) => f64::from((*x) as i32),
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
        Expr::Val(value.into())
    }
}

impl From<i16> for Expr {
    fn from(value: i16) -> Self {
        Expr::Val(value.into())
    }
}

impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        Expr::Val(value.into())
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Val(value.into())
    }
}

impl From<i128> for Expr {
    fn from(value: i128) -> Self {
        Expr::Val(value.into())
    }
}

impl From <u8> for Expr {
    fn from(value: u8) -> Self {
        Expr::Val(value.into())
    }
}

impl From<u16> for Expr {
    fn from(value: u16) -> Self {
        Expr::Val(value.into())
    }
}

impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        Expr::Val(value.into())
    }
}

impl From<u64> for Expr {
    fn from(value: u64) -> Self {
        Expr::Val(value.into())
    }
}

impl From<u128> for Expr {
    fn from(value: u128) -> Self {
        Expr::Val(value as i128)
    }
}

impl From<isize> for Expr {
    fn from(value: isize) -> Self {
        Expr::Val(value as i128)
    }
}

impl From<usize> for Expr {
    fn from(value: usize) -> Self {
        Expr::Val(value as i128)
    }
}

impl From<f32> for Expr {
    fn from(value: f32) -> Self {
        let denominator = 10_000_000;
        let numerator = (value * (denominator as f32)) as i128;

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
        let numerator = (value * (denominator as f64)) as i128;

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

impl From<&Expr> for Expr {
    fn from(value: &Expr) -> Self {
        value.clone()
    }
}

impl From<&i8> for Expr {
    fn from(value: &i8) -> Self {
        Expr::from(*value)
    }
}

impl From<&i16> for Expr {
    fn from(value: &i16) -> Self {
        Expr::from(*value)
    }
}

impl From<&i32> for Expr {
    fn from(value: &i32) -> Self {
        Expr::from(*value)
    }
}

impl From<&i64> for Expr {
    fn from(value: &i64) -> Self {
        Expr::from(*value)
    }
}

impl From<&i128> for Expr {
    fn from(value: &i128) -> Self {
        Expr::from(*value)
    }
}

impl From<&u8> for Expr {
    fn from(value: &u8) -> Self {
        Expr::from(*value)
    }
}

impl From<&u16> for Expr {
    fn from(value: &u16) -> Self {
        Expr::from(*value)
    }
}

impl From<&u32> for Expr {
    fn from(value: &u32) -> Self {
        Expr::from(*value)
    }
}

impl From<&u64> for Expr {
    fn from(value: &u64) -> Self {
        Expr::from(*value)
    }
}

impl From<&u128> for Expr {
    fn from(value: &u128) -> Self {
        Expr::from(*value)
    }
}

impl From<&isize> for Expr {
    fn from(value: &isize) -> Self {
        Expr::from(*value)
    }
}

impl From<&usize> for Expr {
    fn from(value: &usize) -> Self {
        Expr::from(*value)
    }
}

impl From<&f32> for Expr {
    fn from(value: &f32) -> Self {
        Expr::from(*value)
    }
}

impl From<&f64> for Expr {
    fn from(value: &f64) -> Self {
        Expr::from(*value)
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
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

impl Add<i8> for Expr {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<i16> for Expr {
    type Output = Self;

    fn add(self, rhs: i16) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<i32> for Expr {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<i64> for Expr {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<i128> for Expr {
    type Output = Self;

    fn add(self, rhs: i128) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<u8> for Expr {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<u16> for Expr {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<u32> for Expr {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<u64> for Expr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<u128> for Expr {
    type Output = Self;

    fn add(self, rhs: u128) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<isize> for Expr {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<usize> for Expr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<f32> for Expr {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self + Expr::from(rhs)
    }
}

impl Add<f64> for Expr {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Expr::from(rhs)
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

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
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

impl Sub<i8> for Expr {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<i16> for Expr {
    type Output = Self;

    fn sub(self, rhs: i16) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<i32> for Expr {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<i64> for Expr {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<i128> for Expr {
    type Output = Self;

    fn sub(self, rhs: i128) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<u8> for Expr {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<u16> for Expr {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<u32> for Expr {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<u64> for Expr {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<u128> for Expr {
    type Output = Self;

    fn sub(self, rhs: u128) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<isize> for Expr {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<usize> for Expr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<f32> for Expr {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self - Expr::from(rhs)
    }
}

impl Sub<f64> for Expr {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Expr::from(rhs)
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

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = self {
            if x == 1 {
                return rhs;
            } else if x == 0 {
                return Expr::from(0)
            }
        }

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            } else if x == 0 {
                return Expr::from(0)
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

impl Mul<i8> for Expr {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<i16> for Expr {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<i32> for Expr {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<i64> for Expr {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<i128> for Expr {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<u8> for Expr {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<u16> for Expr {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<u32> for Expr {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<u64> for Expr {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<u128> for Expr {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<isize> for Expr {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<usize> for Expr {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<f32> for Expr {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self * Expr::from(rhs)
    }
}

impl Mul<f64> for Expr {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Expr::from(rhs)
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

impl Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            } else {
                if let Expr::Val(y) = self {
                    let gcd = gcd(x, y);

                    return Expr::Div(
                        Box::new(
                            Expr::from(y / gcd)
                        ),
                        Box::new(
                            Expr::from(x / gcd)
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

impl Div<i8> for Expr {
    type Output = Self;

    fn div(self, rhs: i8) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<i16> for Expr {
    type Output = Self;

    fn div(self, rhs: i16) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<i32> for Expr {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<i64> for Expr {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<i128> for Expr {
    type Output = Self;

    fn div(self, rhs: i128) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<u8> for Expr {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<u16> for Expr {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<u32> for Expr {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<u64> for Expr {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<u128> for Expr {
    type Output = Self;

    fn div(self, rhs: u128) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<isize> for Expr {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<usize> for Expr {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<f32> for Expr {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self / Expr::from(rhs)
    }
}

impl Div<f64> for Expr {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / Expr::from(rhs)
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

impl Rem for Expr {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
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

impl Rem<i8> for Expr {
    type Output = Self;

    fn rem(self, rhs: i8) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<i16> for Expr {
    type Output = Self;

    fn rem(self, rhs: i16) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<i32> for Expr {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<i64> for Expr {
    type Output = Self;

    fn rem(self, rhs: i64) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<i128> for Expr {
    type Output = Self;

    fn rem(self, rhs: i128) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<u8> for Expr {
    type Output = Self;

    fn rem(self, rhs: u8) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<u16> for Expr {
    type Output = Self;

    fn rem(self, rhs: u16) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<u32> for Expr {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<u64> for Expr {
    type Output = Self;

    fn rem(self, rhs: u64) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<u128> for Expr {
    type Output = Self;

    fn rem(self, rhs: u128) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<isize> for Expr {
    type Output = Self;

    fn rem(self, rhs: isize) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<usize> for Expr {
    type Output = Self;

    fn rem(self, rhs: usize) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<f32> for Expr {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        self % Expr::from(rhs)
    }
}

impl Rem<f64> for Expr {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        self % Expr::from(rhs)
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

impl BitXor for Expr {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let rhs = Expr::from(rhs);

        if let Expr::Val(x) = rhs {
            if x == 1 {
                return self;
            } else if x == 0 {
                return Expr::from(1);
            }
        }

        if let Expr::Val(x) = self {
            if x == 0 {
                return Expr::from(x);
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

impl BitXor<i8> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: i8) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<i16> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: i16) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<i32> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: i32) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<i64> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: i64) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<i128> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: i128) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<u8> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<u16> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: u16) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<u32> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: u32) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<u64> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: u64) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<u128> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: u128) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<isize> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: isize) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<usize> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<f32> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: f32) -> Self::Output {
        self ^ Expr::from(rhs)
    }
}

impl BitXor<f64> for Expr {
    type Output = Self;

    fn bitxor(self, rhs: f64) -> Self::Output {
        self ^ Expr::from(rhs)
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

impl PartialEq for Expr {
    fn eq(&self, rhs: &Self) -> bool {
        self.eval() == rhs.eval()
    }
}

impl PartialEq<i8> for Expr {
    fn eq(&self, rhs: &i8) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<i16> for Expr {
    fn eq(&self, rhs: &i16) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<i32> for Expr {
    fn eq(&self, rhs: &i32) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<i64> for Expr {
    fn eq(&self, rhs: &i64) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<i128> for Expr {
    fn eq(&self, rhs: &i128) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<u8> for Expr {
    fn eq(&self, rhs: &u8) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<u16> for Expr {
    fn eq(&self, rhs: &u16) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<u32> for Expr {
    fn eq(&self, rhs: &u32) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<u64> for Expr {
    fn eq(&self, rhs: &u64) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<u128> for Expr {
    fn eq(&self, rhs: &u128) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<isize> for Expr {
    fn eq(&self, rhs: &isize) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<usize> for Expr {
    fn eq(&self, rhs: &usize) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<f32> for Expr {
    fn eq(&self, rhs: &f32) -> bool {
        self.eval() == Expr::from(rhs).eval()
    }
}

impl PartialEq<f64> for Expr {
    fn eq(&self, rhs: &f64) -> bool {
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
        let expected_result = ((5.0 * (3.0 + 2.0 - 0.5 / 5.0 * 2.0)) / 31.0 + 55.0) / 23.0;

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
