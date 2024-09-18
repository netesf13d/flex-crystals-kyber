//use std::cmp::{min, max};
use std::default::Default;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul};
// use crate::algebra::arithmetic::PRIME;


//const DEGREE: usize = 255;
//const PRIME: u32 = 7681;
//const MAX_DEGREE: usize = 256;


/// General Kyber Ring element.
/// An element of Z/pZ[X] / (X^n + 1) where n = 2^(n'-1).
#[derive(Clone, Debug, PartialEq)]
pub struct KyberRingElt {
    pub p: u32,
    pub n: usize, // cyclotomic polynomial degree
    pub coefs: Vec<u32>,
}

impl KyberRingElt {
    pub fn new(p: u32, n: usize, mut coefs: Vec<u32>) -> Self {
        coefs.resize(n, 0);
        KyberRingElt {p, n, coefs}
    }
}

impl Default for KyberRingElt {
    fn default() -> Self { KyberRingElt { p: 1, n: 0, coefs: vec![] } }
}

impl Sum for KyberRingElt {
    fn sum<I>(mut iter: I) -> KyberRingElt
    where I: Iterator<Item = KyberRingElt>,
    {
        let mut sum_ = match iter.next() {
            Some(val) => val,
            None => Self::default(),
        };

        while let Some(val) = iter.next() {
            sum_ += val;
        }

        sum_
    }
}

impl Add for KyberRingElt {
    type Output = KyberRingElt;
    fn add(self, rhs: Self) -> Self {
        let mut coefs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            coefs.push((self.coefs[i] + rhs.coefs[i]) % self.p);
        }
        KyberRingElt {p: self.p, n: self.n, coefs: coefs}
    }
}

impl Sub for KyberRingElt {
    type Output = KyberRingElt;
    fn sub(self, rhs: Self) -> Self {
        let mut coefs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            coefs.push((self.coefs[i] + self.p - rhs.coefs[i]) % self.p);
        }
        KyberRingElt {p: self.p, n: self.n, coefs: coefs}
    }
}

impl Neg for KyberRingElt {
    type Output = KyberRingElt;
    fn neg(self) -> Self {
        let mut coefs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            coefs.push((self.p - self.coefs[i]) % self.p);
        }
        KyberRingElt {p: self.p, n: self.n, coefs: coefs}
    }
}

impl Mul for KyberRingElt {
    type Output = KyberRingElt;
    fn mul(self, rhs: Self) -> Self {
        let mut temp = vec![0u32; 2*self.n];
        // Regular polynomial multiplication
        for i in 0..self.n {
            for j in 0..self.n {
                temp[i + j] += (self.coefs[i] * rhs.coefs[j]) % self.p;
            }
        }
        // Quotient with X^n + 1
        let mut coefs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            coefs.push((temp[i] + (self.p-1)*temp[self.n + i]) % self.p);
        }

        KyberRingElt::new(self.p, self.n, coefs)
    }
}

/// Temporary implementation of Mul between &KyberRingElt
/// Should be done using macro in the future
impl<'a, 'b> Mul<&'b KyberRingElt> for &'a KyberRingElt {
    type Output = KyberRingElt;
    fn mul(self, rhs: &'b KyberRingElt) -> KyberRingElt {
        let mut temp = vec![0u32; 2*self.n];
        // Regular polynomial multiplication
        for i in 0..self.n {
            for j in 0..self.n {
                temp[i + j] += (self.coefs[i] * rhs.coefs[j]) % self.p;
            }
        }
        // Quotient with X^n + 1
        let mut coefs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            coefs.push((temp[i] + (self.p-1)*temp[self.n + i]) % self.p);
        }

        KyberRingElt::new(self.p, self.n, coefs)
    }
}


impl AddAssign for KyberRingElt {
    fn add_assign(&mut self, rhs: KyberRingElt) {
        for (i, a) in rhs.coefs.into_iter().enumerate() {
            self.coefs[i] += a;
        }
    }
}

impl SubAssign for KyberRingElt {
    fn sub_assign(&mut self, rhs: KyberRingElt) {
        for (i, a) in rhs.coefs.into_iter().enumerate() {
            self.coefs[i] = (self.coefs[i] + self.p - a) % self.p;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let p1 = KyberRingElt::new(7681, 4, vec![0, 2000, 7680]);
        let p2 = KyberRingElt::new(7681, 4, vec![1, 1000, 2, 1]);
        assert_eq!(
            p1 + p2,
            KyberRingElt::new(7681, 4, vec![1, 3000, 1, 1]),
        );
    }

    #[test]
    fn test_sub() {
        let p1 = KyberRingElt::new(7681, 4, vec![0, 2000, 7680]);
        let p2 = KyberRingElt::new(7681, 4, vec![1, 1000, 2, 1]);
        assert_eq!(
            p1 - p2,
            KyberRingElt::new(7681, 4, vec![7680, 1000, 7678, 7680]),
        );
    }

    #[test]
    fn test_neg() {
        let p1 = KyberRingElt::new(7681, 4, vec![0, 2000, 7680]);
        assert_eq!(
            -p1,
            KyberRingElt::new(7681, 4, vec![0, 5681, 1]),
        );
    }

     #[test]
    fn test_mul_1() {
        let p1 = KyberRingElt::new(7681, 4, vec![1, 0, 0]);
        let p2 = KyberRingElt::new(7681, 4, vec![1, 1000, 2, 1]);
        assert_eq!(
            p1 * p2,
            KyberRingElt::new(7681, 4, vec![1, 1000, 2, 1]),
        );
    }

    #[test]
    fn test_mul_2() {
        let p1 = KyberRingElt::new(7681, 4, vec![0, 2000, 7680]);
        let p2 = KyberRingElt::new(7681, 4, vec![1, 1000, 2, 1]);
        println!("{:?}", p1);
        assert_eq!(
            p1 * p2,
            KyberRingElt::new(7681, 4, vec![5683, 2001, 2939, 3000]),
        );
    }

}