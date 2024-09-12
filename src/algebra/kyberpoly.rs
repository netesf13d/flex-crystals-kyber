use std::cmp::{min, max};
use std::ops::{Add, Mul};
// use crate::algebra::arithmetic::PRIME;


const DEGREE: usize = 255;
const PRIME: u32 = 7681;
const MAX_DEGREE: usize = 256;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KyberRingElt {
    pub coefs: [u32; DEGREE+1],
}


impl KyberRingElt {
    pub fn new(coefs: [u32; DEGREE+1]) -> Self {
        KyberRingElt {coefs}
    }
}



impl Add for KyberRingElt {
    type Output = KyberRingElt;
    fn add(self, rhs: Self) -> Self {
        let mut coefs = [0u32; DEGREE+1];
        for i in 0..=DEGREE {
            coefs[i] = (self.coefs[i] + rhs.coefs[i]) % PRIME;
        }

        KyberRingElt::new(coefs)
    }
}


impl Mul for KyberRingElt {
    type Output = KyberRingElt;
    fn mul(self, rhs: Self) -> Self {
        let mut temp = [0u32; 2*DEGREE+1];
        // Regular polynomia multiplication
        for k in 0..=2*DEGREE+1 {
            let i0 = max(0, k - DEGREE);
            let i1 = min(k, DEGREE);
            for i in i0..=i1 {
                temp[k] += (self.coefs[i] * rhs.coefs[DEGREE-i]) % PRIME;
            }
        }

        // Quotient with X^256 + 1
        let mut coefs: [u32; DEGREE+1] = [temp[DEGREE]; DEGREE+1];
        for i in 0..DEGREE {
            coefs[i] = (temp[i] + (PRIME-1)*temp[i+DEGREE+1]) % PRIME;
        }

        KyberRingElt::new(coefs)
    }
}



/* 
#[test]
fn test_poly_add() {
    let mut x = Polynomial::new(PRIME, [1; MAX_DEGREE]);
    let mut y = Polynomial::new(PRIME, [1; MAX_DEGREE]);
    assert_eq!(x + y, Polynomial::new(PRIME, [2; MAX_DEGREE]));
}
 */