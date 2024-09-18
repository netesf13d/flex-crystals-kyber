use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul};
use std::ops::{Index, IndexMut};

// TODO impl ops for references with macro


#[derive(Clone, Debug, PartialEq)]
pub struct Matrix2D<T>
{
    shape: (usize, usize),
    values: Vec<T>,
}

impl<T> Matrix2D<T> {
    pub fn new(shape: (usize, usize), values: Vec<T>) -> Self {
        assert_eq!(shape.0*shape.1, values.len());
        Matrix2D::<T> {shape, values}
    }

    pub fn matmul(&mut self, mat: &Matrix2D<T>)
    where
        for<'a> &'a T: Mul<Output = T>,
        T: Sum<T>,
    {
        assert_eq!(self.shape.1, mat.shape.0);

        let (n, p, m) = (self.shape.0, self.shape.1, mat.shape.1);
        let mut mprod: Vec<T> = Vec::with_capacity(n*m);
        for i in 0..n {
            for j in 0..m {
                let mut temp: Vec<T> = Vec::with_capacity(p);
                for k in 0..p {
                    temp.push(&self.values[i * p + k] * &mat.values[k * m + j]);
                }
                mprod.push(temp.into_iter().sum());
            }
        }

        self.shape = (n, m);
        self.values = mprod;
    }
}

impl<T> Index<usize> for Matrix2D<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &[T] {
        let start = self.shape.1 * row;
        &self.values[start..start+self.shape.1]
    }
}

impl<T> IndexMut<usize> for Matrix2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = self.shape.1 * row;
        &mut self.values[start..start+self.shape.1]
    }
}

impl<T> Add for Matrix2D<T>
where T: Add<Output = T>,
{
    type Output = Matrix2D<T>;
    fn add(self, rhs: Self) -> Matrix2D<T> {
        assert_eq!(self.shape, rhs.shape);

        let mut it = rhs.values.into_iter();
        let msum = self.values
            .into_iter()
            .map(|x| {x + it.next().unwrap()})
            .collect();

        Matrix2D {shape: self.shape, values: msum}
    }
}

impl<T> Sub for Matrix2D<T>
where T: Sub<Output = T>,
{
    type Output = Matrix2D<T>;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.shape, rhs.shape);

        let mut it = rhs.values.into_iter();
        let mdiff = self.values
            .into_iter()
            .map(|x| {x - it.next().unwrap()})
            .collect();

        Matrix2D {shape: self.shape, values: mdiff}
    }
}

impl<T> Neg for Matrix2D<T>
where T: Neg<Output = T>,
{
    type Output = Matrix2D<T>;
    fn neg(self) -> Self {
        let mneg = self.values
            .into_iter()
            .map(|x| {-x})
            .collect();

        Matrix2D {shape: self.shape, values: mneg}
    }
}

impl<T> AddAssign for Matrix2D<T>
where T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape, rhs.shape);
        for (i, a) in rhs.values.into_iter().enumerate() {
            self.values[i] += a;
        }
    }
}

impl<T> SubAssign for Matrix2D<T>
where T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape, rhs.shape);
        for (i, a) in rhs.values.into_iter().enumerate() {
            self.values[i] -= a;
        }
    }
}


pub fn matmul<T>(lhs: &Matrix2D<T>, rhs: &Matrix2D<T>) -> Matrix2D<T>
where
    for<'a> &'a T: Mul<Output = T>,
    T: Sum<T>,
{
    assert_eq!(lhs.shape.1, rhs.shape.0);

    let (n, p, m) = (lhs.shape.0, lhs.shape.1, rhs.shape.1);
        let mut mprod: Vec<T> = Vec::with_capacity(n*m);
        for i in 0..n {
            for j in 0..m {
                let mut temp: Vec<T> = Vec::with_capacity(p);
                for k in 0..p {
                    temp.push(&lhs.values[i * p + k] * &rhs.values[k * m + j]);
                }
                mprod.push(temp.into_iter().sum());
            }
        }

    Matrix2D {shape: (lhs.shape.0, rhs.shape.1), values: mprod}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let m1 = Matrix2D::<u32> {shape: (2, 2), values: vec![1, 2, 3, 4]};
        let m2 = Matrix2D::<u32> {shape: (2, 2), values: vec![4, 3, 2, 1]};
        assert_eq!(
            m1 + m2,
            Matrix2D::<u32> {shape: (2, 2), values: vec![5, 5, 5, 5]}
        );
    }

    #[test]
    fn test_sub() {
        let m1 = Matrix2D::<i32> {shape: (2, 2), values: vec![1, 2, 3, 4]};
        let m2 = Matrix2D::<i32> {shape: (2, 2), values: vec![4, 3, 2, 1]};
        assert_eq!(
            m1 - m2,
            Matrix2D::<i32> {shape: (2, 2), values: vec![-3, -1, 1, 3]}
        );
    }

    #[test]
    fn test_neg() {
        let m1 = Matrix2D::<i32> {shape: (2, 2), values: vec![1, 2, 3, 4]};
        assert_eq!(
            -m1,
            Matrix2D::<i32> {shape: (2, 2), values: vec![-1, -2, -3, -4]}
        );
    }

    #[test]
    fn test_matmul() {
        let mut m1 = Matrix2D::<u32> {shape: (2, 2), values: vec![1, 2, 3, 4]};
        let m2 = Matrix2D::<u32> {shape: (2, 2), values: vec![4, 3, 2, 1]};
        assert_eq!(
            matmul(&m1, &m2),
            Matrix2D::<u32> {shape: (2, 2), values: vec![8, 5, 20, 13]}
        );

        let m3 = Matrix2D::<u32> {shape: (2, 1), values: vec![4, 3]};
        println!("{:?}", m1.values);
        m1.matmul(&m3);
        assert_eq!(
            m1,
            Matrix2D::<u32> {shape: (2, 1), values: vec![10, 24]}
        );
    }

}




