use crate::precise::expression::Expression;
use std::ops::{ Index, IndexMut };

pub struct Vector2 {
    data: [Expression; 2],
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 {
            data: [
                Expression::value(0),
                Expression::value(0),
            ]
        }
    }

    pub fn x(&self) -> &Expression {
        &self.data[0]
    }

    pub fn x_mut(&mut self) -> &mut Expression {
        &mut self.data[0]
    }

    pub fn y(&self) -> &Expression {
        &self.data[1]
    }

    pub fn y_mut(&mut self) -> &mut Expression {
        &mut self.data[1]
    }

    pub fn component(self, rhs: Self) -> Self {
        Vector2 {
            data: [
                self.data[0].clone() * rhs[0].clone(),
                self.data[1].clone() * rhs[1].clone(),
            ]
        }
    }
}

impl From<Expression> for Vector2 {
    fn from(value: Expression) -> Self {
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
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<i32> for Vector2 {
    fn from(value: i32) -> Self {
        Vector2 {
            data: [
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<f32> for Vector2 {
    fn from(value: f32) -> Self {
        Vector2 {
            data: [
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<f64> for Vector2 {
    fn from(value: f64) -> Self {
        Vector2 {
            data: [
                Expression::from(value),
                Expression::from(value),
            ]
        }
    }
}

impl From<(Expression, Expression)> for Vector2 {
    fn from(value: (Expression, Expression)) -> Self {
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
                Expression::from(x),
                Expression::from(y),
            ]
        }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expression::from(x),
                Expression::from(y),
            ]
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expression::from(x),
                Expression::from(y),
            ]
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from(value: (f64, f64)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                Expression::from(x),
                Expression::from(y),
            ]
        }
    }
}

impl Index<usize> for Vector2 {
    type Output = Expression;

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
    #[test]
    fn new() {

    }

    #[test]
    fn from_expr() {

    }

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
    fn from_expr_expr() {

    }

    #[test]
    fn from_i16_i16() {

    }

    #[test]
    fn from_i32_i32() {

    }

    #[test]
    fn from_f32_f32() {

    }

    #[test]
    fn from_f64_f64() {

    }

    #[test]
    fn x() {

    }

    #[test]
    fn x_mut() {

    }

    #[test]
    fn y() {

    }

    #[test]
    fn y_mut() {

    }

    #[test]
    fn add() {

    }
    
    #[test]
    fn sub() {

    }

    #[test]
    fn dot() {

    }

    #[test]
    fn component() {

    }

    #[test]
    fn scale() {
        
    }

    #[test]
    fn index() {

    }

    #[test]
    fn index_mut() {

    }
}
