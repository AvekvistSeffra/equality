use crate::precise::expression::Expression;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Vector3 {
    data: [Expression; 3],
}

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 {
            data: [
                Expression::from(0),
                Expression::from(0),
                Expression::from(0),
            ],
        }
    }

    pub fn x(&self) -> &Expression {
        &self[0]
    }

    pub fn x_mut(&mut self) -> &mut Expression {
        &mut self[0]
    }

    pub fn y(&self) -> &Expression {
        &self[1]
    }

    pub fn y_mut(&mut self) -> &mut Expression {
        &mut self[1]
    }

    pub fn z(&self) -> &Expression {
        &self[2]
    }

    pub fn z_mut(&mut self) -> &mut Expression {
        &mut self[2]
    }

    pub fn component(self, rhs: Self) -> Self {
        Vector3 {
            data: [
                self[0].clone() * rhs[0].clone(),
                self[1].clone() * rhs[1].clone(),
                self[2].clone() * rhs[2].clone(),
            ]
        }
    }

    pub fn norm(&self) -> Expression {
        ((&self[0] ^ 2) + (&self[1] ^ 2) + (&self[2] ^ 2)) ^ 0.5
    }

    pub fn normalize(self) -> Vector3 {
        let norm = self.norm();
        self * norm
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::from((self[0].clone() + rhs[0].clone(), self[1].clone() + rhs[1].clone(), self[2].clone() + rhs[2].clone()))
    }
}

impl Sub for Vector3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::from((self[0].clone() - rhs[0].clone(), self[1].clone() - rhs[1].clone(), self[2].clone() - rhs[2].clone()))
    }
}

impl Mul for Vector3 {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        self[0].clone() * rhs[0].clone() + self[1].clone() * rhs[1].clone() + self[2].clone() * rhs[2].clone()
    }
}

impl Mul<Expression> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        Vector3::from((self[0].clone() * rhs.clone(), self[1].clone() * rhs.clone(), self[2].clone() * rhs.clone()))
    }
}

impl Mul<i16> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector3 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
            ]
        }
    }
}

impl Mul<i32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector3 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
            ]
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
            ]
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
            ]
        }
    }
}

impl From<Expression> for Vector3 {
    fn from(value: Expression) -> Self {
        Vector3 {
            data: [
                value.clone(),
                value.clone(),
                value,
            ]
        }
    }
}

impl From<i16> for Vector3 {
    fn from(value: i16) -> Self {
        Vector3 {
            data: [
                Expression::from(value),
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<i32> for Vector3 {
    fn from(value: i32) -> Self {
        Vector3 {
            data: [
                Expression::from(value),
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<f32> for Vector3 {
    fn from(value: f32) -> Self {
        Vector3 {
            data: [
                Expression::from(value),
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<f64> for Vector3 {
    fn from(value: f64) -> Self {
        Vector3 {
            data: [
                Expression::from(value),
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<(Expression, Expression, Expression)> for Vector3 {
    fn from(value: (Expression, Expression, Expression)) -> Self {
        let (x, y, z) = value;

        Vector3 {
            data: [
                x,
                y,
                z,
            ]
        }
    }
}

impl From<(i16, i16, i16)> for Vector3 {
    fn from(value: (i16, i16, i16)) -> Self {
        let (x, y, z) = value;

        Vector3 {
            data: [
                Expression::from(x),
                Expression::from(y),
                Expression::from(z),
            ]
        }
    }
}

impl From<(i32, i32, i32)> for Vector3 {
    fn from(value: (i32, i32, i32)) -> Self {
        let (x, y, z) = value;

        Vector3 {
            data: [
                Expression::from(x),
                Expression::from(y),
                Expression::from(z),
            ]
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        let (x, y, z) = value;

        Vector3 {
            data: [
                Expression::from(x),
                Expression::from(y),
                Expression::from(z),
            ]
        }
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        let (x, y, z) = value;

        Vector3 {
            data: [
                Expression::from(x),
                Expression::from(y),
                Expression::from(z),
            ]
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1] && self[2] == rhs[2]
    }
}

impl Index<usize> for Vector3 {
    type Output = Expression;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;
    use crate::precise::expression::Expression;

    #[test]
    fn new() {
        let result = Vector3::new();
        let expected_result = Vector3 {
            data: [
                Expression::from(0),
                Expression::from(0),
                Expression::from(0),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr() {
        let result = Vector3::from(Expression::from(3));
        let expected_result = Vector3 {
            data: [
                Expression::from(3),
                Expression::from(3),
                Expression::from(3),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16() {
        let result = Vector3::from(3_i16);
        let expected_result = Vector3 {
            data: [
                Expression::from(3_i16),
                Expression::from(3_i16),
                Expression::from(3_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Vector3::from(3_i32);
        let expected_result = Vector3 {
            data: [
                Expression::from(3_i32),
                Expression::from(3_i32),
                Expression::from(3_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Vector3::from(3.2_f32);
        let expected_result = Vector3 {
            data: [
                Expression::from(3.2_f32),
                Expression::from(3.2_f32),
                Expression::from(3.2_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Vector3::from(3.2_f64);
        let expected_result = Vector3 {
            data: [
                Expression::from(3.2_f64),
                Expression::from(3.2_f64),
                Expression::from(3.2_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr_expr_expr() {
        let result = Vector3::from((Expression::from(3), Expression::from(2), Expression::from(4)));
        let expected_result = Vector3 {
            data: [
                Expression::from(3),
                Expression::from(2),
                Expression::from(4),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16_i16_i16() {
        let result = Vector3::from((3_i16, 2_i16, 4_i16));
        let expected_result = Vector3 {
            data: [
                Expression::from(3_i16),
                Expression::from(2_i16),
                Expression::from(4_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32_i32_i32() {
        let result = Vector3::from((3_i32, 2_i32, 4_i32));
        let expected_result = Vector3 {
            data: [
                Expression::from(3_i32),
                Expression::from(2_i32),
                Expression::from(4_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32_f32_f32() {
        let result = Vector3::from((3.2_f32, 2.1_f32, 5.5_f32));
        let expected_result = Vector3 {
            data: [
                Expression::from(3.2_f32),
                Expression::from(2.1_f32),
                Expression::from(5.5_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64_f64_f64() {
        let result = Vector3::from((3.2_f64, 2.1_f64, 5.5_f64));
        let expected_result = Vector3 {
            data: [
                Expression::from(3.2_f64),
                Expression::from(2.1_f64),
                Expression::from(5.5_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x() {
        let result = Vector3::from((4, 3, 1)).x().clone();
        let expected_result = Expression::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x_mut() {
        let result = Vector3::from((4, 3, 1)).x_mut().clone();
        let expected_result = Expression::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y() {
        let result = Vector3::from((4, 3, 1)).y().clone();
        let expected_result = Expression::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y_mut() {
        let result = Vector3::from((4, 3, 1)).y_mut().clone();
        let expected_result = Expression::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z() {
        let result = Vector3::from((4, 3, 1)).z().clone();
        let expected_result = Expression::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z_mut() {
        let result = Vector3::from((4, 3, 1)).z_mut().clone();
        let expected_result = Expression::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn add() {
        let result = Vector3::from((3, 5, 2)) + Vector3::from(2);
        let expected_result = Vector3::from((5, 7, 4));

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn sub() {
        let result = Vector3::from((3, 5, 2)) - Vector3::from(2);
        let expected_result = Vector3::from((1, 3, 0));

        assert_eq!(result, expected_result);

    }

    #[test]
    fn dot() {
        let result = Vector3::from((3, 5, 2)) * Vector3::from(2);
        let expected_result = 20;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn component() {
        let result = Vector3::from((3, 5, 2)).component(Vector3::from((2, 4, 3)));
        let expected_result = Vector3::from((6, 20, 6));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_expr() {
        let result = Vector3::from((3, 5, 2)) * Expression::from(2);
        let expected_result = Vector3::from((6, 10, 4));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i16() {
        let result = Vector3::from((3, 5, 2)) * 2_i16;
        let expected_result = Vector3::from((6, 10, 4));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i32() {
        let result = Vector3::from((3, 5, 2)) * 2_i32;
        let expected_result = Vector3::from((6, 10, 4));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f32() {
        let result = Vector3::from((3, 5, 2)) * 2_f32;
        let expected_result = Vector3::from((6, 10, 4));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f64() {
        let result = Vector3::from((3, 5, 2)) * 2_f64;
        let expected_result = Vector3::from((6, 10, 4));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn norm() {
        let result = Vector3::from((3, 4, 2)).norm() ^ 2;
        let expected_result = 3.0_f32 * 3.0_f32 + 4.0_f32 * 4.0_f32 + 2.0_f32 * 2.0_f32;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn normalize() {
        let vector = Vector3::from((3, 4, 2));
        let norm = vector.norm();

        let result = vector.normalize();
        let expected_result = Vector3::from((3.0 / norm.clone(), 4.0 / norm.clone(), 2.0 / norm.clone()));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn index() {
        let vector = Vector3::from((3, 5, 2));

        let result1 = &vector[0];
        let result2 = &vector[1];
        let result3 = &vector[2];
        
        let expected_result1 = Expression::from(3);
        let expected_result2 = Expression::from(5);
        let expected_result3 = Expression::from(2);

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
        assert_eq!(result3, expected_result3);
    }

    #[test]
    #[allow(unused_mut)]
    fn index_mut() {
        let vector = Vector3::from((3, 5, 2));

        let mut result1 = &vector[0];
        let mut result2 = &vector[1];
        let mut result3 = &vector[2];
        
        let expected_result1 = Expression::from(3);
        let expected_result2 = Expression::from(5);
        let expected_result3 = Expression::from(2);

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
        assert_eq!(result3, expected_result3);
    }
}
