use crate::precise::expression::Expression;
use std::ops::{ Index, IndexMut };

pub struct Matrix3 {
    data: [Expression; 9],
}

impl Index<usize> for Matrix3 {
    type Output = Expression;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
