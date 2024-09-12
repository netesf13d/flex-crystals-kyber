mod algebra;

use std::env;

use crate::algebra::arithmetic::{IntMod, PRIME};


fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        println!("Error");
        std::process::exit(1);
    }

    let mut x = IntMod {value: 7, modulus: PRIME};
    x.add(1000);
    x.add(8000);
    assert_eq!(x, IntMod {value: 1326, modulus: PRIME});

    



}

