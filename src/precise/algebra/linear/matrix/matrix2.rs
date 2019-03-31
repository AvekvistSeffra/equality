use crate::precise::expression::Expression;
use std::ops::{ Index, IndexMut };

pub struct Matrix2 {
    data: [Expression; 4],
}

impl Index<usize> for Matrix2 {
    type Output = Expression;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
