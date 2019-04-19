use crate::precise::expression::Expr;
use std::ops::{ Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Matrix3 {
    data: [Expr; 9],
}

impl Index<usize> for Matrix3 {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
