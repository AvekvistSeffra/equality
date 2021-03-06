use crate::precise::expression::Expr;
use std::ops::{ Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Matrix4 {
    data: [Expr; 16],
}

impl Index<usize> for Matrix4 {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
