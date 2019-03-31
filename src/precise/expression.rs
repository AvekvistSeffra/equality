use std::fmt;
use fmt::{ Display, Formatter };
use std::ops::{ Add, Sub, Mul, Div, Rem, BitXor };
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Value(i32),
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Exponent,
}

impl From<i32> for Operation {
    fn from(value: i32) -> Self {
        Operation::Value(value)
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    operation: Operation,
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

impl Expression {
    pub fn value(value: i32) -> Expression {
        Expression {
            operation: Operation::Value(value),
            lhs: None,
            rhs: None,
        }
    }

    pub fn evaluate(&self) -> f64 {
        match self.operation {
            Operation::Value(x) => x as f64,
            Operation::Addition => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate() + rhs.evaluate()
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            Operation::Subtraction => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate() - rhs.evaluate()
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            Operation::Multiplication => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate() * rhs.evaluate()
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            Operation::Division => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate() / rhs.evaluate()
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            Operation::Modulus => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate() % rhs.evaluate()
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            Operation::Exponent => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        lhs.evaluate().powf(rhs.evaluate())
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
        }
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Addition,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Subtraction,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Multiplication,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Rem for Expression {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Modulus,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl BitXor for Expression {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Exponent,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl BitXor<i32> for Expression {
    type Output = Self;

    fn bitxor(self, rhs: i32) -> Self::Output {
        Expression {
            operation: Operation::Exponent,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(Expression::value(rhs))),
        }
    }
}

impl BitXor<f32> for Expression {
    type Output = Self;

    fn bitxor(self, rhs: f32) -> Self::Output {
        let denominator = 10_000_000;
        let rhs = (rhs * (denominator as f32)) as i32;
        let gcd = crate::utils::greatest_common_divisor(rhs, denominator);

        Expression {
            operation: Operation::Exponent,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(Expression {
                operation: Operation::Division,
                lhs: Some(Box::new(Expression::value(rhs / gcd))),
                rhs: Some(Box::new(Expression::value(denominator / gcd))),
            })),
        }
    }
}

impl From<i16> for Expression {
    fn from(value: i16) -> Self {
        Expression::value(value as i32)
    }
}

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Expression::value(value)
    }
}

impl From<f32> for Expression {
    fn from(value: f32) -> Self {
        let denominator = 10_000_000;
        let value = (value * (denominator as f32)) as i32;
        let gcd = crate::utils::greatest_common_divisor(value, denominator);

        Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(Expression::value(value / gcd))),
            rhs: Some(Box::new(Expression::value(denominator / gcd))),
        }
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        let denominator = 10_000_000;
        let value = (value * (denominator as f64)) as i32;
        let gcd = crate::utils::greatest_common_divisor(value, denominator);

        Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(Expression::value(value / gcd))),
            rhs: Some(Box::new(Expression::value(denominator / gcd))),
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let lhs = self.evaluate();
        let rhs = rhs.evaluate();

        if lhs < rhs {
            Some(Ordering::Less)
        } else if lhs > rhs {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = self.evaluate();
        let rhs = rhs.evaluate();

        if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, rhs: &Self) -> bool {
        self.evaluate() == rhs.evaluate()
    }
}

impl PartialEq<i16> for Expression {
    fn eq(&self, rhs: &i16) -> bool {
        self.evaluate() == (*rhs as f64)
    }
}

impl PartialEq<i32> for Expression {
    fn eq(&self, rhs: &i32) -> bool {
        self.evaluate() == (*rhs as f64)
    }
}

impl PartialEq<f32> for Expression {
    fn eq(&self, rhs: &f32) -> bool {
        self.evaluate() == (*rhs as f64)
    }
}

impl PartialEq<f64> for Expression {
    fn eq(&self, rhs: &f64) -> bool {
        self.evaluate() == *rhs
    }
}

impl Eq for Expression {}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.operation {
            Operation::Value(x) => write!(f, "{}", x),
            Operation::Addition => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} + {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
            Operation::Subtraction => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} - {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
            Operation::Multiplication => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} * {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
            Operation::Division => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} / {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
            Operation::Modulus => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} % {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
            Operation::Exponent => {
                if let Some(lhs) = &self.lhs {
                    if let Some(rhs) = &self.rhs {
                        write!(f, "({} ^ {})", lhs, rhs)
                    } else {
                        write!(f, "")
                    }
                } else {
                    write!(f, "")
                }
            },
        }
    }    
}

#[cfg(test)]
mod operation_tests {
    use super::Operation;

    #[test]
    fn from() {
        
    }
}

#[cfg(test)]
mod expr_tests {
    use super::Expression;
    use super::Operation;

    #[test]
    fn from_i16() {
        
    }

    #[test]
    fn from_i32() {

    }

    #[test]
    fn from_f32() {

    }

    #[test]
    fn from_f64() {

    }

    #[test]
    fn value() {

    }

    #[test]
    fn evaluate() {
        let test_expression = Expression {
            operation: Operation::Addition,
            lhs: Some(Box::new(Expression {
                        operation: Operation::Value(2),
                        lhs: None,
                        rhs: None,
                    }
                )
            ),
            rhs: Some(Box::new(Expression {
                        operation: Operation::Value(3),
                        lhs: None,
                        rhs: None,
                    }
                )
            ),
        };

        let result = 5.0;

        assert_eq!(test_expression.evaluate(), result);
    }

    #[test]
    fn add() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs + rhs;
        let expected_result = 21;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn sub() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs - rhs;
        let expected_result = -11;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn mul() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs * rhs;
        let expected_result = 80;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn div() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs / rhs;
        let expected_result = 5.0 / 16.0;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn rem() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs % rhs;
        let expected_result = 5 % 16;

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn pow() {
        let lhs = Expression {
            operation: Operation::Value(5),
            lhs: None,
            rhs: None,
        };

        let rhs = Expression {
            operation: Operation::Value(16),
            lhs: None,
            rhs: None,
        };

        let result = lhs ^ rhs;
        let expected_result = 5_f64.powf(16.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn pow_i32() {

    }

    #[test]
    fn pow_f32() {

    }
}
