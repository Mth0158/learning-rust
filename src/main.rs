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
    // tutorial_data_types();
    // tutorial_math();
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

fn tutorial_data_types() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize
    println!("Max u64: {}", u64::MAX);
    println!("Max f64: {}", f64::MAX);
    println!("Max i64: {}", i64::MAX);

    let is_true = true;
    let my_grade = 'A';
}

fn tutorial_math() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}