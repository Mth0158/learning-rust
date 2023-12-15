#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y
}

fn old_main() {
    // Thanks Derek Banas for this incredible tutorial
    // You can find it at https://www.youtube.com/watch?v=ygL_xcavzQ4

    // tutorial_variables();
    // tutorial_constants();
    // tutorial_data_types();
    // tutorial_math();
    // tutorial_conditions();
    // tutorial_arrays();
    // tutorial_tuples();
    // tutorial_strings();
    // tutorial_casting();
    // tutorial_enums();
    // tutorial_vectors();
    // say_hello();
    // get_sum(3, 4);
    // println!("get_sum_2: {}", get_sum_2(3, 3));
    // let num_list = vec![1,2,3,4,5];
    // println!("Sum of list = {}", sum_list(&num_list));
    // println!("{}", get_sum_gen(1, 3));
    // println!("{}", get_sum_gen(1.5, 3.2));
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

fn tutorial_casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u3: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn tutorial_enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Every hates Monday"),
        Day::Tuesday => println!("Every hates Tuesday"),
        Day::Wednesday => println!("Every hates Wednesday"),
        Day::Thursday => println!("Every hates Thursday"),
        Day::Friday => println!("Every hates Friday"),
        Day::Saturday => println!("Every hates Saturday"),
        Day::Sunday => println!("Every hates Sunday"),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}

fn tutorial_vectors() {
    // Are like arrays
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value")
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}

// -----

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string: {}", x);
    x
}

fn change_str(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // play_with_ownership();
    // play_with_hasmaps();
    // play_with_structs();
    // play_with_shapes();
    // order_food();
    play_with_errors_and_files();
}

fn play_with_ownership() {
    let mut str1 = String::from("Mathieu");
    // print_str(str1);
    let str3 = print_return_str(str1.clone());
    println!("str3 = {}", str3);
    change_str(&mut str1);
}

fn play_with_hasmaps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k , v);
    }

    println!("Length: {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero :/"),
        }
    }
}

fn play_with_structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("55 Main St"),
        balance:  234.5
    };
    bob.address = String::from("48 Main st");
    println!("Bob's new address: {}", bob.address);

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };
}

fn play_with_shapes() {
    const PI: f32 = 3.14;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec area: {}", rec.area());
    println!("Circ area: {}", circ.area());
}

fn play_with_errors_and_files() {
    // panic!("Terrible error");
    // let lil_arr= [1, 2];
    // println!("{}", lil_arr[10]);
    let path = "lines.text";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let outpu2 = File::create("rand.txt");
    let output2 = match outpu2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            }
            _other_error => panic!("Problem opening file: {:?}", error)
        }
    };
}
