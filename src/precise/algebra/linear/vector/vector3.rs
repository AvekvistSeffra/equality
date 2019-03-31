use crate::precise::expression::Expression;
use std::ops::{ Index, IndexMut };

pub struct Vector3 {
    data: [Expression; 3],
}

impl Vector3 {
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
}

impl Index<usize> for Vector3 {
    type Output = Expression;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
