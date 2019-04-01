use crate::precise::expression::Expression;
use std::ops::{ Mul, Index, IndexMut };

pub struct Vector4 {
    data: [Expression; 4],
}

impl Vector4 {
    pub fn new() -> Vector4 {
        Vector4 {
            data: [
                Expression::from(0),
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

    pub fn w(&self) -> &Expression {
        &self[3]
    }

    pub fn w_mut(&mut self) -> &mut Expression {
        &mut self[3]
    }

    pub fn norm(&self) -> Expression {
        ((self[0].clone() ^ 2) + (self[1].clone() ^ 2) + (self[2].clone() ^ 2) + (self[3].clone() ^ 2)) ^ (Expression::from(1) / Expression::from(2))
    }

    pub fn normalize(self) -> Vector4 {
        let norm = self.norm();
        self * norm
    }
}

impl Mul<Expression> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
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

impl Index<usize> for Vector4 {
    type Output = Expression;

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
    use crate::precise::expression::Expression;

    #[test]
    fn new() {
        let result = Vector4::new();
        let expected_result = Vector4 {
            data: [
                Expression::from(0),
                Expression::from(0),
                Expression::from(0),
                Expression::from(0),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr() {
        let result = Vector4::from(Expression::from(3));
        let expected_result = Vector4 {
            data: [
                Expression::from(3),
                Expression::from(3),
                Expression::from(3),
                Expression::from(3),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16() {
        let result = Vector4::from(3_i16);
        let expected_result = Vector4 {
            data: [
                Expression::from(3_i16),
                Expression::from(3_i16),
                Expression::from(3_i16),
                Expression::from(3_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32() {
        let result = Vector4::from(3_i32);
        let expected_result = Vector4 {
            data: [
                Expression::from(3_i32),
                Expression::from(3_i32),
                Expression::from(3_i32),
                Expression::from(3_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32() {
        let result = Vector4::from(3.2_f32);
        let expected_result = Vector4 {
            data: [
                Expression::from(3.2_f32),
                Expression::from(3.2_f32),
                Expression::from(3.2_f32),
                Expression::from(3.2_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Vector4::from(3.2_f64);
        let expected_result = Vector4 {
            data: [
                Expression::from(3.2_f64),
                Expression::from(3.2_f64),
                Expression::from(3.2_f64),
                Expression::from(3.2_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_expr_expr_expr_expr() {
        let result = Vector4::from((Expression::from(3), Expression::from(2), Expression::from(4), Expression::from(1)));
        let expected_result = Vector4 {
            data: [
                Expression::from(3),
                Expression::from(2),
                Expression::from(4),
                Expression::from(1),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i16_i16_i16_i16() {
        let result = Vector4::from((3_i16, 2_i16, 4_i16, 1_i16));
        let expected_result = Vector4 {
            data: [
                Expression::from(3_i16),
                Expression::from(2_i16),
                Expression::from(4_i16),
                Expression::from(1_i16),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_i32_i32_i32_i32() {
        let result = Vector4::from((3_i32, 2_i32, 4_i32, 1_i32));
        let expected_result = Vector4 {
            data: [
                Expression::from(3_i32),
                Expression::from(2_i32),
                Expression::from(4_i32),
                Expression::from(1_i32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f32_f32_f32_f32() {
        let result = Vector4::from((3.2_f32, 2.1_f32, 5.5_f32, 1.3_f32));
        let expected_result = Vector4 {
            data: [
                Expression::from(3.2_f32),
                Expression::from(2.1_f32),
                Expression::from(5.5_f32),
                Expression::from(1.3_f32),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64_f64_f64_f64() {
        let result = Vector4::from((3.2_f64, 2.1_f64, 5.5_f64, 1.3_f64));
        let expected_result = Vector4 {
            data: [
                Expression::from(3.2_f64),
                Expression::from(2.1_f64),
                Expression::from(5.5_f64),
                Expression::from(1.3_f64),
            ]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x() {
        let result = Vector4::from((4, 3, 1, 2)).x().clone();
        let expected_result = Expression::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x_mut() {
        let result = Vector4::from((4, 3, 1, 2)).x_mut().clone();
        let expected_result = Expression::from(4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y() {
        let result = Vector4::from((4, 3, 1, 2)).y().clone();
        let expected_result = Expression::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn y_mut() {
        let result = Vector4::from((4, 3, 1, 2)).y_mut().clone();
        let expected_result = Expression::from(3);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z() {
        let result = Vector4::from((4, 3, 1, 2)).z().clone();
        let expected_result = Expression::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn z_mut() {
        let result = Vector4::from((4, 3, 1, 2)).z_mut().clone();
        let expected_result = Expression::from(1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn w() {
        let result = Vector4::from((4, 3, 1, 2)).w().clone();
        let expected_result = Expression::from(2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn w_mut() {
        let result = Vector4::from((4, 3, 1, 2)).w_mut().clone();
        let expected_result = Expression::from(2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn add() {
        let result = Vector4::from((4, 5, 2, 1)) + Vector4::from(2);
        let expected_result = Vector4::from((5, 7, 4, 3));

        assert_eq!(result, expected_result);
    }
    
    #[test]
    fn sub() {
        let result = Vector4::from((4, 5, 2, 1)) - Vector4::from(2);
        let expected_result = Vector4::from((1, 3, 0, -1));

        assert_eq!(result, expected_result);

    }

    #[test]
    fn dot() {
        let result = Vector4::from((4, 5, 2, 1)) * Vector4::from(2);
        let expected_result = 22;

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
        let result = Vector4::from((4, 5, 2, 1)) * Expression::from(2);
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
        let result = Vector3::from((3, 4, 2, 1)).norm();
        let expected_result = (3.0 * 3.0 + 4.0 * 4.0 + 2.0 * 2.0 + 1.0 * 1.0).sqrt();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn normalize() {
        let norm = (3.0 * 3.0 + 4.0 * 4.0 + 2.0 * 2.0 + 1.0 * 1.0).sqrt();

        let result = Vector3::from((3, 4, 2, 1)).normalize();
        let expected_result = Vector3::from((3.0 / norm, 4.0 / norm, 2.0 / norm, 1.0 / norm));

        assert_eq!(result, expected_result);
    }


    #[test]
    fn index() {
        let vector = Vector4::from((4, 5, 2, 1));

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
        let vector = Vector4::from((4, 5, 2, 1));

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
