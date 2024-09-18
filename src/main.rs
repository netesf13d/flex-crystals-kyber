mod algebra;

use std::env;

//use crate::algebra::arithmetic::{IntMod, PRIME};


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        println!("Error");
        std::process::exit(1);
    }
    let a: Vec<i32> = Vec::with_capacity(3);
    let c: i32 = a.into_iter().sum();
    println!("{:?}", c);

/*     let b: i32 = 25;
    let c: i32 = 42;
    let a: i32 = &b + &c;
    println!("{}", a); */


    



}

