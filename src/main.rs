extern crate num;

use std::io;
use std::io::Result;
use std::io::prelude::*;
use std::str::FromStr;

use num::BigUint;
use num::One;
use num::Zero;

fn main() {
    let mut n = [BigUint::zero(), BigUint::zero()];
    for i in 0..2 {
        loop {
            match prompt::<BigUint>(">> ") {
                Err(..) => println!("Please enter valid input"),
                Ok(input) => {
                    n[i] = input;
                    break;
                },
            }
        }
    }

    println!("{}", permutations(&n[0], &n[1]));
}

// Given two unsigned integers n and r, compute P(n, r).
fn permutations(n: &BigUint, r: &BigUint) -> BigUint {
    factorial(n) / factorial(&(n - r))
}

// Given an unsigned integer, compute the factorial.
fn factorial(n: &BigUint) -> BigUint {
    let mut res = BigUint::one();

    for i in num::range_inclusive(BigUint::one(), n.clone()) {
        res = res * i;
    }

    res
}

// Prompt for input on stdout, reading in a line from stdin.
fn prompt<T>(sym: &str) -> Result<T>
    where T: FromStr
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    try!(stdout.write(sym.as_bytes()));
    try!(stdout.flush());

    let mut buf = String::new();
    try!(stdin.read_line(&mut buf));
    buf.pop(); // Remove newline.

    if let Ok(ret) = buf.parse() {
        Ok(ret)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "bad input"))
    }
}
