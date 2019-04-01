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

impl From<i16> for Operation {
    fn from(value: i16) -> Self {
        Operation::Value(value as i32)
    }
}

impl From<i32> for Operation {
    fn from(value: i32) -> Self {
        Operation::Value(value)
    }
}

impl PartialEq for Operation {
    fn eq(&self, rhs: &Self) -> bool {
        if let Operation::Value(x) = self {
            if let Operation::Value(y) = rhs {
                x == y
            } else {
                false
            }
        } else {
            match self {
                Operation::Addition => {
                    match rhs {
                        Operation::Addition => true,
                        _ => false,
                    }
                },
                Operation::Subtraction => {
                    match rhs {
                        Operation::Subtraction => true,
                        _ => false,
                    }
                },
                Operation::Multiplication => {
                    match rhs {
                        Operation::Multiplication => true,
                        _ => false,
                    }
                },
                Operation::Division => {
                    match rhs {
                        Operation::Division => true,
                        _ => false,
                    }
                },
                Operation::Modulus => {
                    match rhs {
                        Operation::Modulus => true,
                        _ => false,
                    }
                },
                _ => false,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    operation: Operation,
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

impl Expression {
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

impl Add for &Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Addition,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs.clone())),
        }
    }
}

impl Add<Expression> for &Expression {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Self::Output {
        Expression {
            operation: Operation::Addition,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Add<&Expression> for Expression {
    type Output = Self;

    fn add(self, rhs: &Expression) -> Self::Output {
        Expression {
            operation: Operation::Addition,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs.clone())),
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

impl Sub for &Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Subtraction,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs.clone())),
        }
    }
}

impl Sub<Expression> for &Expression {
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Self::Output {
        Expression {
            operation: Operation::Subtraction,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Sub<&Expression> for Expression {
    type Output = Self;

    fn sub(self, rhs: &Expression) -> Self::Output {
        Expression {
            operation: Operation::Subtraction,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs.clone())),
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

impl Mul for &Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression {
            operation: Operation::Multiplication,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs.clone())),
        }
    }
}

impl Mul<Expression> for &Expression {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression {
            operation: Operation::Multiplication,
            lhs: Some(Box::new(self.clone())),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl Mul<&Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Expression {
            operation: Operation::Multiplication,
            lhs: Some(Box::new(self)),
            rhs: Some(Box::new(rhs.clone())),
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
            rhs: Some(Box::new(Expression::from(rhs))),
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
                lhs: Some(Box::new(Expression::from(rhs / gcd))),
                rhs: Some(Box::new(Expression::from(denominator / gcd))),
            })),
        }
    }
}

impl From<i16> for Expression {
    fn from(value: i16) -> Self {
        Expression::from(value as i32)
    }
}

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Expression {
            operation: Operation::Value(value),
            lhs: None,
            rhs: None,
        }
    }
}

impl From<f32> for Expression {
    fn from(value: f32) -> Self {
        let denominator = 10_000_000;
        let value = (value * (denominator as f32)) as i32;
        let gcd = crate::utils::greatest_common_divisor(value, denominator);

        Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(Expression::from(value / gcd))),
            rhs: Some(Box::new(Expression::from(denominator / gcd))),
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
            lhs: Some(Box::new(Expression::from(value / gcd))),
            rhs: Some(Box::new(Expression::from(denominator / gcd))),
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

impl PartialEq<Expression> for &Expression {
    fn eq(&self, rhs: &Expression) -> bool {
        self.evaluate() == rhs.evaluate()
    }
}

impl PartialEq<&Expression> for Expression {
    fn eq(&self, rhs: &&Expression) -> bool {
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
    fn from_i16() {
        let result = Operation::from(3_i16);
        let expected_result = Operation::Value(3_i32);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Operation::from(3_i32);
        let expected_result = Operation::Value(3_i32);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn eq_value_value() {
        let op1 = Operation::from(0);
        let op2 = Operation::from(0);

        assert_eq!(op1, op2);
    }

    #[test]
    fn ne_value_value() {
        let op1 = Operation::from(0);
        let op2 = Operation::from(1);

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_value_addition() {
        let op1 = Operation::from(0);
        let op2 = Operation::Addition;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_value_subtraction() {
        let op1 = Operation::from(0);
        let op2 = Operation::Subtraction;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_value_multiplication() {
        let op1 = Operation::from(0);
        let op2 = Operation::Multiplication;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_value_division() {
        let op1 = Operation::from(0);
        let op2 = Operation::Division;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_value_modulus() {
        let op1 = Operation::from(0);
        let op2 = Operation::Modulus;

        assert_ne!(op1, op2);
    }

    #[test]
    fn eq_addition_addition() {
        let op1 = Operation::Addition;
        let op2 = Operation::Addition;

        assert_eq!(op1, op2);
    }

    #[test]
    fn ne_addition_subtraction() {
        let op1 = Operation::Addition;
        let op2 = Operation::Subtraction;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_addition_multiplication() {
        let op1 = Operation::Addition;
        let op2 = Operation::Multiplication;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_addition_division() {
        let op1 = Operation::Addition;
        let op2 = Operation::Division;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_addition_modulus() {
        let op1 = Operation::Addition;
        let op2 = Operation::Modulus;

        assert_ne!(op1, op2);
    }

    #[test]
    fn eq_subtraction_subtraction() {
        let op1 = Operation::Subtraction;
        let op2 = Operation::Subtraction;

        assert_eq!(op1, op2);
    }

    #[test]
    fn ne_subtraction_multiplication() {
        let op1 = Operation::Subtraction;
        let op2 = Operation::Multiplication;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_subtraction_division() {
        let op1 = Operation::Subtraction;
        let op2 = Operation::Division;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_subtraction_modulus() {
        let op1 = Operation::Subtraction;
        let op2 = Operation::Modulus;

        assert_ne!(op1, op2);
    }

    #[test]
    fn eq_multiplication_multiplication() {
        let op1 = Operation::Multiplication;
        let op2 = Operation::Multiplication;

        assert_eq!(op1, op2);
    }

    #[test]
    fn ne_multiplication_division() {
        let op1 = Operation::Multiplication;
        let op2 = Operation::Division;

        assert_ne!(op1, op2);
    }

    #[test]
    fn ne_multiplication_modulus() {
        let op1 = Operation::Multiplication;
        let op2 = Operation::Modulus;

        assert_ne!(op1, op2);
    }

    #[test]
    fn eq_division_division() {
        let op1 = Operation::Division;
        let op2 = Operation::Division;

        assert_eq!(op1, op2);
    }

    #[test]
    fn ne_division_modulus() {
        let op1 = Operation::Division;
        let op2 = Operation::Modulus;

        assert_ne!(op1, op2);
    }

    #[test]
    fn eq_modulus_modulus() {
        let op1 = Operation::Modulus;
        let op2 = Operation::Modulus;

        assert_eq!(op1, op2);
    }
}

#[cfg(test)]
mod expr_tests {
    use super::Expression;
    use super::Operation;

    #[test]
    fn from_i16() {
        let result = Expression::from(3_i16);
        let expected_result = Expression {
            operation: Operation::Value(3),
            lhs: None,
            rhs: None,
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Expression::from(3_i32);
        let expected_result = Expression {
            operation: Operation::Value(3),
            lhs: None,
            rhs: None,
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Expression::from(3.2_f32);
        let expected_result = Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(Expression::from(16))),
            rhs: Some(Box::new(Expression::from(5))),
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Expression::from(3.2_f64);
        let expected_result = Expression {
            operation: Operation::Division,
            lhs: Some(Box::new(Expression::from(16))),
            rhs: Some(Box::new(Expression::from(5))),
        };

        assert_eq!(result, expected_result);
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

        let result = test_expression.evaluate();
        let expected_result = 5.0;

        assert_eq!(result, expected_result);
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
        let result = Expression::from(5) ^ Expression::from(16);
        let expected_result = 5_f64.powf(16.0);

        assert_eq!(result, expected_result);
    }
}
