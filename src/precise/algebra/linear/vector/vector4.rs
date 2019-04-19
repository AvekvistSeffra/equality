use crate::precise::expression::Expr;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct Vector4 {
    data: [Expr; 4],
}

impl std::fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector4 {
    pub fn new() -> Vector4 {
        Vector4 {
            data: [
                Expr::from(0),
                Expr::from(0),
                Expr::from(0),
                Expr::from(0),
            ],
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

    pub fn z(&self) -> &Expr {
        &self[2]
    }

    pub fn z_mut(&mut self) -> &mut Expr {
        &mut self[2]
    }

    pub fn w(&self) -> &Expr {
        &self[3]
    }

    pub fn w_mut(&mut self) -> &mut Expr {
        &mut self[3]
    }

    pub fn component(self, rhs: Self) -> Self {
        Vector4 {
            data: [
                self[0].clone() * rhs[0].clone(),
                self[1].clone() * rhs[1].clone(),
                self[2].clone() * rhs[2].clone(),
                self[3].clone() * rhs[3].clone(),
            ]
        }
    }

    pub fn norm(&self) -> Expr {
        ((*self.x() ^ 2) + (*self.y() ^ 2) + (*self.z() ^ 2) + (*self.w() ^ 2)) ^ 0.5
    }

    pub fn normalize(self) -> Vector4 {
        self.clone() * (1 / self.norm())
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector4::from((self[0].clone() + rhs[0].clone(), self[1].clone() + rhs[1].clone(), self[2].clone() + rhs[2].clone(), self[3].clone() + rhs[3].clone()))
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector4::from((self[0].clone() - rhs[0].clone(), self[1].clone() - rhs[1].clone(), self[2].clone() - rhs[2].clone(), self[3].clone() - rhs[3].clone()))
    }
}

impl Mul for Vector4 {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        self[0].clone() * rhs[0].clone() + self[1].clone() * rhs[1].clone() + self[2].clone() * rhs[2].clone() + self[3].clone() * rhs[3].clone()
    }
}

impl Mul<Expr> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Expr) -> Self::Output {
        Vector4 {
            data: [
                self[0].clone() * rhs.clone(),
                self[1].clone() * rhs.clone(),
                self[2].clone() * rhs.clone(),
                self[3].clone() * rhs,
            ]
        }
    }
}

impl Mul<i16> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector4 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
                self[3].clone() * rhs,
            ]
        }
    }
}

impl Mul<i32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector4 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
                self[3].clone() * rhs,
            ]
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
                self[3].clone() * rhs,
            ]
        }
    }
}

impl Mul<f64> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector4 {
            data: [
                self[0].clone() * rhs,
                self[1].clone() * rhs,
                self[2].clone() * rhs,
                self[3].clone() * rhs,
            ]
        }
    }
}

impl From<Expr> for Vector4 {
    fn from(value: Expr) -> Self {
        Vector4 {
            data: [
                value.clone(),
                value.clone(),
                value.clone(),
                value,
            ]
        }
    }
}

impl From<i16> for Vector4 {
    fn from(value: i16) -> Self {
        Vector4 {
            data: [
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<i32> for Vector4 {
    fn from(value: i32) -> Self {
        Vector4 {
            data: [
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<f32> for Vector4 {
    fn from(value: f32) -> Self {
        Vector4 {
            data: [
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<f64> for Vector4 {
    fn from(value: f64) -> Self {
        Vector4 {
            data: [
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
                Expr::from(value),
            ]
        }
    }
}

impl From<(Expr, Expr, Expr, Expr)> for Vector4 {
    fn from(value: (Expr, Expr, Expr, Expr)) -> Self {
        let (x, y, z, w) = value;

        Vector4 {
            data: [
                x,
                y,
                z,
                w,
            ]
        }
    }
}

impl From<(i16, i16, i16, i16)> for Vector4 {
    fn from(value: (i16, i16, i16, i16)) -> Self {
        let (x, y, z, w) = value;

        Vector4 {
            data: [
                Expr::from(x),
                Expr::from(y),
                Expr::from(z),
                Expr::from(w),
            ]
        }
    }
}

impl From<(i32, i32, i32, i32)> for Vector4 {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        let (x, y, z, w) = value;

        Vector4 {
            data: [
                Expr::from(x),
                Expr::from(y),
                Expr::from(z),
                Expr::from(w),
            ]
        }
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        let (x, y, z, w) = value;

        Vector4 {
            data: [
                Expr::from(x),
                Expr::from(y),
                Expr::from(z),
                Expr::from(w),
            ]
        }
    }
}

impl From<(f64, f64, f64, f64)> for Vector4 {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        let (x, y, z, w) = value;

        Vector4 {
            data: [
                Expr::from(x),
                Expr::from(y),
                Expr::from(z),
                Expr::from(w),
            ]
        }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1] && self[2] == rhs[2] && self[3] == rhs[3]
    }
}

impl Index<usize> for Vector4 {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Vector4;
    use crate::precise::expression::Expr;

    #[test]
    fn new() {
        let result = Vector4::new();
        let expected_result = Vector4 {
            data: [
                Expr::from(0),
                Expr::from(0),
                Expr::from(0),
                Expr::from(0),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr() {
        let result = Vector4::from(Expr::from(3));
        let expected_result = Vector4 {
            data: [
                Expr::from(3),
                Expr::from(3),
                Expr::from(3),
                Expr::from(3),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16() {
        let result = Vector4::from(3_i16);
        let expected_result = Vector4 {
            data: [
                Expr::from(3_i16),
                Expr::from(3_i16),
                Expr::from(3_i16),
                Expr::from(3_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Vector4::from(3_i32);
        let expected_result = Vector4 {
            data: [
                Expr::from(3_i32),
                Expr::from(3_i32),
                Expr::from(3_i32),
                Expr::from(3_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Vector4::from(3.2_f32);
        let expected_result = Vector4 {
            data: [
                Expr::from(3.2_f32),
                Expr::from(3.2_f32),
                Expr::from(3.2_f32),
                Expr::from(3.2_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Vector4::from(3.2_f64);
        let expected_result = Vector4 {
            data: [
                Expr::from(3.2_f64),
                Expr::from(3.2_f64),
                Expr::from(3.2_f64),
                Expr::from(3.2_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr_expr_expr_expr() {
        let result = Vector4::from((Expr::from(3), Expr::from(2), Expr::from(4), Expr::from(1)));
        let expected_result = Vector4 {
            data: [
                Expr::from(3),
                Expr::from(2),
                Expr::from(4),
                Expr::from(1),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16_i16_i16_i16() {
        let result = Vector4::from((3_i16, 2_i16, 4_i16, 1_i16));
        let expected_result = Vector4 {
            data: [
                Expr::from(3_i16),
                Expr::from(2_i16),
                Expr::from(4_i16),
                Expr::from(1_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32_i32_i32_i32() {
        let result = Vector4::from((3_i32, 2_i32, 4_i32, 1_i32));
        let expected_result = Vector4 {
            data: [
                Expr::from(3_i32),
                Expr::from(2_i32),
                Expr::from(4_i32),
                Expr::from(1_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32_f32_f32_f32() {
        let result = Vector4::from((3.2_f32, 2.1_f32, 5.5_f32, 1.3_f32));
        let expected_result = Vector4 {
            data: [
                Expr::from(3.2_f32),
                Expr::from(2.1_f32),
                Expr::from(5.5_f32),
                Expr::from(1.3_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64_f64_f64_f64() {
        let result = Vector4::from((3.2_f64, 2.1_f64, 5.5_f64, 1.3_f64));
        let expected_result = Vector4 {
            data: [
                Expr::from(3.2_f64),
                Expr::from(2.1_f64),
                Expr::from(5.5_f64),
                Expr::from(1.3_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x() {
        let result = Vector4::from((4, 3, 1, 2)).x().clone();
        let expected_result = Expr::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x_mut() {
        let result = Vector4::from((4, 3, 1, 2)).x_mut().clone();
        let expected_result = Expr::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y() {
        let result = Vector4::from((4, 3, 1, 2)).y().clone();
        let expected_result = Expr::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y_mut() {
        let result = Vector4::from((4, 3, 1, 2)).y_mut().clone();
        let expected_result = Expr::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z() {
        let result = Vector4::from((4, 3, 1, 2)).z().clone();
        let expected_result = Expr::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z_mut() {
        let result = Vector4::from((4, 3, 1, 2)).z_mut().clone();
        let expected_result = Expr::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn w() {
        let result = Vector4::from((4, 3, 1, 2)).w().clone();
        let expected_result = Expr::from(2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn w_mut() {
        let result = Vector4::from((4, 3, 1, 2)).w_mut().clone();
        let expected_result = Expr::from(2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn add() {
        let result = Vector4::from((4, 5, 2, 1)) + Vector4::from(2);
        let expected_result = Vector4::from((6, 7, 4, 3));

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn sub() {
        let result = Vector4::from((4, 5, 2, 1)) - Vector4::from(2);
        let expected_result = Vector4::from((2, 3, 0, -1));

        assert_eq!(result, expected_result);

    }

    #[test]
    fn dot() {
        let result = Vector4::from((4, 5, 2, 1)) * Vector4::from(2);
        let expected_result = 24;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn component() {
        let result = Vector4::from((4, 5, 2, 1)).component(Vector4::from((2, 4, 3, 5)));
        let expected_result = Vector4::from((8, 20, 6, 5));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_expr() {
        let result = Vector4::from((4, 5, 2, 1)) * Expr::from(2);
        let expected_result = Vector4::from((8, 10, 4, 2));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i16() {
        let result = Vector4::from((4, 5, 2, 1)) * 2_i16;
        let expected_result = Vector4::from((8, 10, 4, 2));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_i32() {
        let result = Vector4::from((4, 5, 2, 1)) * 2_i32;
        let expected_result = Vector4::from((8, 10, 4, 2));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f32() {
        let result = Vector4::from((4, 5, 2, 1)) * 2_f32;
        let expected_result = Vector4::from((8, 10, 4, 2));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scale_f64() {
        let result = Vector4::from((4, 5, 2, 1)) * 2_f64;
        let expected_result = Vector4::from((8, 10, 4, 2));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn norm() {
        let result = Vector4::from((3, 4, 2, 1)).norm() ^ 2;
        let expected_result = 3.0_f32 * 3.0_f32 + 4.0_f32 * 4.0_f32 + 2.0_f32 * 2.0_f32 + 1.0_f32 * 1.0_f32;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn normalize() {
        let vector = Vector4::from((3, 4, 2, 1));
        let norm = vector.norm();

        let result = vector.normalize();
        let expected_result = Vector4::from((3.0 / norm.clone(), 4.0 / norm.clone(), 2.0 / norm.clone(), 1.0 / norm));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn index() {
        let vector = Vector4::from((4, 5, 2, 1));

        let result1 = &vector[0];
        let result2 = &vector[1];
        let result3 = &vector[2];
        let result4 = &vector[3];
        
        let expected_result1 = Expr::from(4);
        let expected_result2 = Expr::from(5);
        let expected_result3 = Expr::from(2);
        let expected_result4 = Expr::from(1);

        assert_eq!(*result1, expected_result1);
        assert_eq!(*result2, expected_result2);
        assert_eq!(*result3, expected_result3);
        assert_eq!(*result4, expected_result4);
    }

    #[test]
    #[allow(unused_mut)]
    fn index_mut() {
        let vector = Vector4::from((4, 5, 2, 1));

        let mut result1 = &vector[0];
        let mut result2 = &vector[1];
        let mut result3 = &vector[2];
        let mut result4 = &vector[3];
        
        let expected_result1 = Expr::from(4);
        let expected_result2 = Expr::from(5);
        let expected_result3 = Expr::from(2);
        let expected_result4 = Expr::from(1);

        assert_eq!(*result1, expected_result1);
        assert_eq!(*result2, expected_result2);
        assert_eq!(*result3, expected_result3);
        assert_eq!(*result4, expected_result4);
    }
}
