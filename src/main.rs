#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Thanks Derek Banas for this incredible tutorial
    // You can find it at https://www.youtube.com/watch?v=ygL_xcavzQ4

    // tutorial_variables();
    // tutorial_constants();
}

fn tutorial_variables() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn tutorial_constants() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
