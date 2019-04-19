use crate::precise::expression::Expr;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct Vector2 {
    data: [Expr; 2],
}

impl std::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self[0], self[1])
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 {
            data: [
                Expr::from(0),
                Expr::from(0),
            ]
        }
    }

    pub fn x(&self) -> &Expr {
        &self[0]
    }

    pub fn x_mut(&mut self) -> &mut Expr {
        &mut self[0]
    }

    pub fn y(&self) -> &Expr {
        &self[1]
    }

    pub fn y_mut(&mut self) -> &mut Expr {
        &mut self[1]
    }

    pub fn component(self, rhs: Self) -> Self {
        Vector2 {
            data: [
                self[0].clone() * rhs[0].clone(),
                self[1].clone() * rhs[1].clone(),
            ]
        }
    }

    pub fn norm(&self) -> Expr {
        ((*self.x() ^ 2) + (*self.y() ^ 2)) ^ 0.5
    }

    pub fn normalize(self) -> Vector2 {
        self.clone() * (1 / self.norm())
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::from((self[0].clone() + rhs[0].clone(), self[1].clone() + rhs[1].clone()))
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::from((self[0].clone() - rhs[0].clone(), self[1].clone() - rhs[1].clone()))
    }
}

impl Mul for Vector2 {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        self[0].clone() * rhs[0].clone() + self[1].clone() * rhs[1].clone()
    }
}

impl Mul<Expr> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Expr) -> Self::Output {
        Vector2::from((self[0].clone() * rhs.clone(), self[1].clone() * rhs))
    }
}

impl Mul<i16> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector2 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
            ]
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
            ]
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
            ]
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
            ]
        }
    }
}

impl From<Expr> for Vector2 {
    fn from(value: Expr) -> Self {
        Vector2 {
            data: [
                value.clone(),
                value,
            ]
        }
    }
}

impl From<i16> for Vector2 {
    fn from(value: i16) -> Self {
        Vector2 {
            data: [
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<i32> for Vector2 {
    fn from(value: i32) -> Self {
        Vector2 {
            data: [
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<f32> for Vector2 {
    fn from(value: f32) -> Self {
        Vector2 {
            data: [
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<f64> for Vector2 {
    fn from(value: f64) -> Self {
        Vector2 {
            data: [
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<(Expr, Expr)> for Vector2 {
    fn from(value: (Expr, Expr)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                x,
                y,
            ]
        }
    }
}

impl From<(i16, i16)> for Vector2 {
    fn from(value: (i16, i16)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expr::from(x),
                Expr::from(y),
            ]
        }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expr::from(x),
                Expr::from(y),
            ]
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expr::from(x),
                Expr::from(y),
            ]
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from(value: (f64, f64)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expr::from(x),
                Expr::from(y),
            ]
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1]
    }
}

impl Index<usize> for Vector2 {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Vector2;
    use crate::precise::expression::Expr;

    #[test]
    fn new() {
        let result = Vector2::new();
        let expected_result = Vector2 {
            data: [
                Expr::from(0),
                Expr::from(0),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr() {
        let result = Vector2::from(Expr::from(3));
        let expected_result = Vector2 {
            data: [
                Expr::from(3),
                Expr::from(3),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16() {
        let result = Vector2::from(3_i16);
        let expected_result = Vector2 {
            data: [
                Expr::from(3_i16),
                Expr::from(3_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Vector2::from(3_i32);
        let expected_result = Vector2 {
            data: [
                Expr::from(3_i32),
                Expr::from(3_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Vector2::from(3.2_f32);
        let expected_result = Vector2 {
            data: [
                Expr::from(3.2_f32),
                Expr::from(3.2_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Vector2::from(3.2_f64);
        let expected_result = Vector2 {
            data: [
                Expr::from(3.2_f64),
                Expr::from(3.2_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr_expr() {
        let result = Vector2::from((Expr::from(3), Expr::from(2)));
        let expected_result = Vector2 {
            data: [
                Expr::from(3),
                Expr::from(2),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16_i16() {
        let result = Vector2::from((3_i16, 2_i16));
        let expected_result = Vector2 {
            data: [
                Expr::from(3_i16),
                Expr::from(2_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32_i32() {
        let result = Vector2::from((3_i32, 2_i32));
        let expected_result = Vector2 {
            data: [
                Expr::from(3_i32),
                Expr::from(2_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32_f32() {
        let result = Vector2::from((3.2_f32, 2.1_f32));
        let expected_result = Vector2 {
            data: [
                Expr::from(3.2_f32),
                Expr::from(2.1_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64_f64() {
        let result = Vector2::from((3.2_f64, 2.1_f64));
        let expected_result = Vector2 {
            data: [
                Expr::from(3.2_f64),
                Expr::from(2.1_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x() {
        let result = Vector2::from((4, 3))[0].clone();
        let expected_result = Expr::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x_mut() {
        let result = Vector2::from((4, 3)).x_mut().clone();
        let expected_result = Expr::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y() {
        let result = Vector2::from((4, 3))[1].clone();
        let expected_result = Expr::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y_mut() {
        let result = Vector2::from((4, 3)).y_mut().clone();
        let expected_result = Expr::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn add() {
        let result = Vector2::from((3, 5)) + Vector2::from(2);
        let expected_result = Vector2::from((5, 7));

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn sub() {
        let result = Vector2::from((3, 5)) - Vector2::from(2);
        let expected_result = Vector2::from((1, 3));

        assert_eq!(result, expected_result);

    }

    #[test]
    fn dot() {
        let result = Vector2::from((3, 5)) * Vector2::from(2);
        let expected_result = 16;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn component() {
        let result = Vector2::from((3, 5)).component(Vector2::from((2, 4)));
        let expected_result = Vector2::from((6, 20));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_expr() {
        let result = Vector2::from((3, 5)) * Expr::from(2);
        let expected_result = Vector2::from((6, 10));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i16() {
        let result = Vector2::from((3, 5)) * 2_i16;
        let expected_result = Vector2::from((6, 10));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i32() {
        let result = Vector2::from((3, 5)) * 2_i32;
        let expected_result = Vector2::from((6, 10));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f32() {
        let result = Vector2::from((3, 5)) * 2_f32;
        let expected_result = Vector2::from((6, 10));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f64() {
        let result = Vector2::from((3, 5)) * 2_f64;
        let expected_result = Vector2::from((6, 10));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn norm() {
        let result = Vector2::from((6, 8)).norm();
        let expected_result = 10;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn normalize() {
        let vector = Vector2::from((3, 4));
        let norm = vector.norm();

        let result = vector.normalize();
        let expected_result = Vector2::from((3 / norm.clone(), 4 / norm.clone()));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn index() {
        let vector = Vector2::from((3, 5));

        let result1 = &vector[0];
        let result2 = &vector[1];
        
        let expected_result1 = Expr::from(3);
        let expected_result2 = Expr::from(5);

        assert_eq!(*result1, expected_result1);
        assert_eq!(*result2, expected_result2);
    }

    #[test]
    #[allow(unused_mut)]
    fn index_mut() {
        let vector = Vector2::from((3, 5));

        let mut result1 = &vector[0];
        let mut result2 = &vector[1];
        
        let expected_result1 = Expr::from(3);
        let expected_result2 = Expr::from(5);

        assert_eq!(*result1, expected_result1);
        assert_eq!(*result2, expected_result2);
    }
}
