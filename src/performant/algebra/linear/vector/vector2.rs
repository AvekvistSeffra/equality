use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use serde_derive::{ Serialize, Deserialize };

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    data: [f64; 2],
}

impl std::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self[0], self[1])
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { 
            data: [
                0.0,
                0.0,
            ],
        }
    }

    pub fn x(&self) -> &f64 {
        &self.data[0]
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.data[0]
    }

    pub fn y(&self) -> &f64 {
        &self.data[1]
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.data[1]
    }

    pub fn component(self, rhs: Self) -> Self {
        Vector2 {
            data: [
                self[0] * rhs[0],
                self[1] * rhs[1],
            ],
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y()).powf(0.5)
    }

    pub fn normalize(self) -> Vector2 {
        self * (1.0 / self.norm())
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            data: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
            ],
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            data: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
            ],
        }
    }
}

impl Mul for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {
            data: [
                self.x() * rhs,
                self.y() * rhs,
            ],
        }
    }
}

impl From<f64> for Vector2 {
    fn from(value: f64) -> Self {
        Vector2 {
            data: [
                value,
                value,
            ],
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from(value: (f64, f64)) -> Self {
        let (x, y) = value;

        Vector2 {
            data: [
                x,
                y,
            ],
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x() == rhs.x() && self.y() == rhs.y()
    }
}

impl Index<usize> for Vector2 {
    type Output = f64;

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
    use super::Vector2;

    #[test]
    fn new() {
        let result = Vector2::new();
        let expected_result = Vector2 {
            data: [
                0.0,
                0.0,
            ],
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64() {
        let result = Vector2::from(3.2);
        let expected_result = Vector2 {
            data: [
                3.2,
                3.2,
            ],
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_f64_f64() {
        let result = Vector2::from((3.2, 5.6));
        let expected_result = Vector2 {
            data: [
                3.2,
                5.6,
            ],
        };

        assert_eq!(result, expected_result);
    }
}
