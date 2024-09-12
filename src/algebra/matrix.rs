use std::ops::{Add, Mul}

use ndarray::Array2;

use crate::algebra::kyberpoly::KyberRingElt;


struct Matrix2D<T> {
    shape: (usize, usize),
    values: Vec<T>,
}

impl Add<T: Add> for Matrix2D {
    type Output = Matrix2D<T>;
    pub fn add(self, rhs: Self) -> Self {
        assert_eq!(self.shape, other.shape);

        let mut msum: Vec<T> = Vec::with_capacity(self.shape.0*self.shape.1);
        for k in 0..self.shape0*self.shape.1 {
            msum[k] = self.mvalues[k] + rhs.values[k];
        }

        Matrix2D {shape: self.shape, values: msum}
    }
}

impl Mul<T: Add + Mul> for Matrix2D {
    type Output = Matrix2D<T>;
    pub fn add(self, rhs: Self) -> Self {
        assert_eq!(self.shape.1, rhs.shape.0);

        let mut mprod: Vec<T> = Vec::with_capacity(self.shape.0*rhs.shape.1);
        let temp: Vec<T> = Vec::with_capacity(self.shape.1);
        let n = rhs.shape.1;
        for k in 0..self.shape0*self.shape.1 {
            for i in 0..self.shape.1 {
                temp[i] = self.values[k / n + i] * rhs.values[i * n + k % n]
            }
            mprod[k] = temp.iter().sum();
        }

        Matrix2D {shape: (self.shape.0, rhs.shape.1), values: mprod}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let m1 = Matrix2D<u32> {vshape: (2, 2), values: vec![1, 2, 3, 4]};
        let m2 = Matrix2D<u32> {vshape: (2, 2), values: vec![4, 3, 2, 1]};
        assert_eq!(
            m1 + m2,
            Matrix2D<u32> {vshape: (2, 2), values: vec![5, 5, 5, 5]}
        );
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix2D<u32> {vshape: (2, 2), values: vec![1, 2, 3, 4]};
        let m2 = Matrix2D<u32> {vshape: (2, 2), values: vec![4, 3, 2, 1]};
        assert_eq!(
            m1 * m2,
            Matrix2D<u32> {vshape: (2, 2), values: vec![8, 6, 20, 13]}
        );

        let m3 = Matrix2D<u32> {vshape: (2, 1), values: vec![4, 3]};
        assert_eq!(
            m1 * m2,
            Matrix2D<u32> {vshape: (2, 2), values: vec![10, 24]}
        );
    }

}




