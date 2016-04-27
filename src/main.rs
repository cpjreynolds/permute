extern crate num;

use std::io;
use std::io::Result;
use std::io::prelude::*;
use std::str::FromStr;

use num::BigUint;
use num::One;
use num::Zero;

fn main() {
    let choice = prompt_choice();
    let (n, r) = prompt_nums();
    match choice {
        Choice::C => {
            let ans = combination(&n, &r);
            println!("C({}, {}) = {}", n, r, ans);
        },
        Choice::P => {
            let ans = permutation(&n, &r);
            println!("P({}, {}) = {}", n, r, ans);
        },
    }
}

fn prompt_nums() -> (BigUint, BigUint) {
    let n;
    let r;

    loop {
        match prompt::<BigUint>("Enter `n`\n>> ") {
            Ok(input) => {
                n = input;
                break;
            },
            Err(..) => println!("Please enter a valid integer.\n"),
        }
    }
    loop {
        match prompt::<BigUint>("Enter `r`\n>> ") {
            Ok(input) => {
                r = input;
                break;
            },
            Err(..) => println!("Please enter a valid integer.\n"),
        }
    }

    (n, r)
}

fn prompt_choice() -> Choice {
    loop {
        match prompt::<Choice>("Combinations or permutations? [c/p]\n>> ") {
            Ok(c) => return c,
            Err(..) => println!("Please enter a valid choice.\n"),
        }
    }
}

// Given two unsigned integers n and r, compute C(n, r).
fn combination(n: &BigUint, r: &BigUint) -> BigUint {
    factorial(n) / (factorial(r) * factorial(&(n - r)))
}

// Given two unsigned integers n and r, compute P(n, r).
fn permutation(n: &BigUint, r: &BigUint) -> BigUint {
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

    match buf.parse() {
        Ok(ret) => Ok(ret),
        Err(..) => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
    }
}

enum Choice {
    C,
    P,
}

impl FromStr for Choice {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "c" || s == "C" {
            Ok(Choice::C)
        } else if s == "p" || s == "P" {
            Ok(Choice::P)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"))
        }
    }
}

