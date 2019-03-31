use std::ops::{ Index, IndexMut };

pub struct Vector4 {
    data: [f64; 4],
}

impl Vector4 {
    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn w(&self) -> f64 {
        self.data[3]
    }
}

impl Index<usize> for Vector4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[index]
    }
}
