use crate::precise::expression::Expr;
use std::ops::{ Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Matrix2 {
    data: [Expr; 4],
}

impl Index<usize> for Matrix2 {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
