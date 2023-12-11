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
    // tutorial_conditions();
    // tutorial_arrays();
    // tutorial_tuples();
    tutorial_strings();
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

fn tutorial_conditions() {
    let age = 8;
    if (age >= 1) && (age >= 18) {
        println!("Hey you have an important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Hey you have an important birthday 2");
    } else if (age >= 65) {
        println!("Hey you have an important birthday 3");
    } else {
        println!("Not birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Not birthday"),
        21 | 50 => println!("Not birthday"),
        65..=i32::MAX => println!("Not birthday"),
        _ => println!("All other cases")
    };

    let my_age = 18;
    let voting_age = 18;
    match  my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Equal => println!("Can vote"),
        Ordering::Greater => println!("Can vote"),
    };
}

fn tutorial_arrays() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx+= 1;
            continue;
        }

        if arr_2[loop_idx] == 5 {
            break;
        }

        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while loop_idx < arr_2.len() {
        println!("Array: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val: {}", val);
    }
}

fn tutorial_tuples() {
    let my_tuple: (u8, String, f64) = (29, "Mathieu".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);

    let(v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
}

fn tutorial_strings() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string ";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_array1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; // will make st6 unaivable (but not st7)
    for char in st8.bytes() {
        println!("{}", char);
    }
}
