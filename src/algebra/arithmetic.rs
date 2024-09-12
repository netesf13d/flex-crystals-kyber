//use std::ops::{Add, Mul, Sub};


pub static PRIME: u16 = 7681;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntMod {
    pub value: u32,
    pub modulus: u16,
}

impl IntMod {
    pub fn new(value: u32, modulus: u16) -> Self {
        IntMod {
            value: value % modulus as u32,
            modulus: modulus }
    }

    pub fn add(&mut self, other: u32) -> () {
        let p = self.modulus as u32;
        self.value = (self.value + other) % p;
    }

    pub fn sub(&mut self, other: u32) -> () {
        let p = self.modulus as u32;
        let neg: u32 = p - (other % p);
        self.value = (self.value + neg) % p;
    }

    pub fn mul(&mut self, other: u32) -> () {
        let p = self.modulus as u32;
        self.value = (self.value * (other % p)) % p;
    }

    pub fn pow(&mut self, other: u32) -> () {
        let mut sq = self.clone();
        let mut modexp = IntMod::new(1, self.modulus);
        let mut x = other;
        while x > 0 {
            if x & 1 == 1 {modexp.mul(sq.value.clone());}
            sq.mul(sq.value);
            x = x >> 1;
        }
        self.value = modexp.value;
    }
}


/// test modular addition
#[test]
fn test_intmod_add() {
    let mut x = IntMod {value: 7, modulus: PRIME};
    x.add(1000);
    assert_eq!(x, IntMod {value: 1007, modulus: PRIME});
    x.add(8000);
    assert_eq!(x, IntMod {value: 1326, modulus: PRIME});
    x.add(1 << 30);
    assert_eq!(x, IntMod {value: 798, modulus: PRIME});
}

/// test substraction
#[test]
fn test_intmod_sub() {
    let mut x = IntMod {value: 798, modulus: PRIME};
    x.sub(1000);
    assert_eq!(x, IntMod {value: 7479, modulus: PRIME});
    x.sub(7000);
    assert_eq!(x, IntMod {value: 479, modulus: PRIME});
    x.sub(700_000);
    assert_eq!(x, IntMod {value: 7131, modulus: PRIME});
}

/// test multiplication
#[test]
fn test_intmod_mul() {
    let mut x = IntMod {value: 7131, modulus: PRIME};
    x.mul(3);
    assert_eq!(x, IntMod {value: 6031, modulus: PRIME});
    x.mul(1<<20);
    assert_eq!(x, IntMod {value: 2531, modulus: PRIME});
}

/// test addition
#[test]
fn test_intmod_pow() {
    let mut x = IntMod {value: 2531, modulus: PRIME};
    x.pow(1000);
    assert_eq!(x, IntMod {value: 6517, modulus: PRIME});
    x.pow(x.value.clone());
    assert_eq!(x, IntMod {value: 1372, modulus: PRIME});
}








