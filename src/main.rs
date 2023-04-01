use std::io::Write;

mod c02;
mod c03;
mod c04;
mod c05;

use c02::*;
use c03::*;
use c04::*;
use c05::*;

fn main() {
/*
    // C02
    c02();
    // C03
    date_tomorrow_with_year();
    temperature_conversion();
    generalised_converter();
    additional_taxes();
    // C04
    println!("fibonacci(10) = {}", fibonacci(10));
    printable_ascii();
    simple_character_graph();
    count_lines_chars();
    count_lines_chars_words();
    collatz_conjecture();
    next_prime();
    sum_of_previous_prime();
    cal_log();
*/
    // C05
    // println!("max_2_ints(1, 2) = {}", max_2_ints(1, 2));
/*
    let mut user_input = String::new();
    print!("Enter 4 integers separated by spaces:");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut numbers = Vec::new();
    for i in user_input.split_whitespace() {
        numbers.push(i);
    }
    let a = numbers[0].parse::<i128>().unwrap();
    let b = numbers[1].parse::<i128>().unwrap();
    let c = numbers[2].parse::<i128>().unwrap();
    let d = numbers[3].parse::<i128>().unwrap();
    println!("max_4_ints({}, {}, {}, {}) = {}", a, b, c, d, max_4_ints(a, b, c, d));
*/
/*
    let mut user_input = String::new();
    print!("Enter 4 integers separated by spaces:");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut numbers = Vec::new();
    for i in user_input.split_whitespace() {
        numbers.push(i);
    }
    let a = numbers[0].parse::<i128>().unwrap();
    let b = numbers[1].parse::<i128>().unwrap();
    let c = numbers[2].parse::<i128>().unwrap();
    println!("median_3_ints({}, {}, {}) = {}", a, b, c, median_3_ints(a, b, c));
*/
/*
    let mut user_input = String::new();
    print!("Enter base and power:");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut numbers = Vec::new();
    for i in user_input.split_whitespace() {
        numbers.push(i);
    }
    let base = numbers[0].parse::<i128>().unwrap();
    let pow = numbers[1].parse::<i128>().unwrap();
    println!("int_pows({}, {}) = {}", base, pow, int_pows(base, pow));
*/
/*
    let mut user_input = String::new();
    print!("Enter n and k:");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut numbers = Vec::new();
    for i in user_input.split_whitespace() {
        numbers.push(i);
    }
    let n = numbers[0].parse::<i128>().unwrap();
    let k = numbers[1].parse::<i128>().unwrap();
    println!("n_choose_k({}, {}) = {}", n, k, n_choose_k(n, k));
*/
/*
    let mut user_input = String::new();
    print!("Enter two integers: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut numbers = Vec::new();
    for i in user_input.split_whitespace() {
        numbers.push(i);
    }
    let a = numbers[0].parse::<i128>().unwrap();
    let b = numbers[1].parse::<i128>().unwrap();
    if amicable_pair(a, b) {
        println!("{} and {} are an amicable pairs", a, b);
    } else {
        println!("{} and {} are not an amicable pairs", a, b);
    }
*/
    let mut user_input = String::new();
    print!("Enter n: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let n = user_input.trim().parse::<i128>().unwrap();
    println!("{} gives sequence sum of {:.10}", n, sum_of_sequence(n));
}
