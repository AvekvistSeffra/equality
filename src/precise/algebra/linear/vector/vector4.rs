use crate::precise::expression::Expression;
use std::ops::{ Mul, Index, IndexMut };

pub struct Vector4 {
    data: [Expression; 4],
}

impl Vector4 {
    pub fn new() -> Vector4 {
        Vector4 {
            data: [
                Expression::value(0),
                Expression::value(0),
                Expression::value(0),
                Expression::value(0),
            ],
        }
    }

    pub fn from(x: f32, y: i32, z: i32, w: i32) -> Vector4 {
        Vector4 {
            data: [
                Expression::from(x),
                Expression::from(y),
                Expression::from(z),
                Expression::from(w),
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

    pub fn z(&self) -> &Expression {
        &self.data[2]
    }

    pub fn z_mut(&mut self) -> &mut Expression {
        &mut self.data[2]
    }

    pub fn w(&self) -> &Expression {
        &self.data[3]
    }

    pub fn w_mut(&mut self) -> &mut Expression {
        &mut self.data[3]
    }

    pub fn norm(&self) -> Expression {
        ((self.data[0].clone() ^ 2) + (self.data[1].clone() ^ 2) + (self.data[2].clone() ^ 2) + (self.data[3].clone() ^ 2)) ^ (Expression::value(1) / Expression::value(2))
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
                self.data[0].clone() * rhs.clone(),
                self.data[1].clone() * rhs.clone(),
                self.data[2].clone() * rhs.clone(),
                self.data[3].clone() * rhs,
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
    #[test]
    fn new() {

    }

    #[test]
    fn from() {

    }
}