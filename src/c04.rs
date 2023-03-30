use std::io::{Write,Read};
fn main() {
    // Ex4.03
    /* let (mut days, mut spores): (u32,u32) = (0,2);
    while spores < 1000000 {
        days += 1;
        if days < 3 {
            continue;
        } else {
            spores = fibonacci(days);
            println!("After {} days, there will be {} spores.", days, spores);
        }
    } */
    // printable_ascii();
    // simple_character_graph();
    // count_lines_chars();
    // count_lines_chars_words();
    // collatz_conjecture();
    // next_prime();
    // sum_of_previous_prime();
    cal_log();
}

// Ex4.03
#[allow(dead_code)]
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Ex4.04
#[allow(dead_code)]
fn printable_ascii() {
    let (mut _i, mut _j): (i8,i8) = (0,0);
    println!("           +0  +1  +2  +3  +4  +5  +6  +7");
    println!("        +--------------------------------");
    // NOTE: In rust, `..` are used to represent range,
    // .step_by() means the step of the range.
    for _i in (32..=126).step_by(8) {
        print!("{:7} |", _i);
        for _j in 0..8 {
            let c = _i + _j;
            if c <= 126 {
                print!("{:4}",c);
            }
        }
        println!();
    }
}

// Ex4.05
// use std::io::{Write, Read};
#[allow(dead_code)]
fn simple_character_graph() {
    let mut input = String::new();
    // we assume user could have multiple inputs and are separate by space
    print!("Enter numbers: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");
    // as the inputs are multiple, we need to split them
    // not sure should use vector or array, so I use vector as it's more flexible
    let mut numbers: Vec<i32> = Vec::new();
    for number in input.split_whitespace() {
        numbers.push(number.parse::<i32>().unwrap());
    }
    for number in numbers {
        print!("{:2} |", number);
        for _ in 0..number {
            print!("*");
        }
        println!();
    }
}

// Ex4.06
#[allow(dead_code)]
fn count_lines_chars() {
    println!("Enter text:");
    let mut user_input = String::new();
    // NOTE: as we are couting lines, the noraml read_lines() is not suitable
    // so we use read_to_string() to read all the inputs
    std::io::stdin()
        .read_to_string(&mut user_input)
        .expect("Error reading input!");
    // count line numbers
    let mut _lines = 0;
    for _ in user_input.lines() {
        _lines += 1;
    }
    let mut _chars = 0;
    for _ in user_input.chars() {
        _chars += 1;
    }
    println!("Lines: {:7}", _lines);
    println!("Chars: {:7}", _chars);

}

#[allow(dead_code)]
fn count_lines_chars_words() {
    println!("Enter text:");
    let mut user_input = String::new();
    // NOTE: as we are couting lines, the noraml read_lines() is not suitable
    // so we use read_to_string() to read all the inputs
    std::io::stdin()
        .read_to_string(&mut user_input)
        .expect("Error reading input!");

    // count line numbers
    let mut _lines = 0;
    for _ in user_input.lines() {
        _lines += 1;
    }
    println!("Lines: {:7}", _lines);

    // count words
    let mut _words = 0;
    let mut _is_word = false;
    for ch in user_input.chars() {
        if ch == ' ' || ch == ',' || ch == ';' || ch == '-' {
            _is_word = true;
        }
        if _is_word && ch != ' ' {
            _words += 1;
            _is_word = false;
        }
    }
    println!("Words: {:7}", _words);

    // count chars
    let mut _chars = 0;
    for _ in user_input.chars() {
        _chars += 1;
    }
    println!("Chars: {:7}", _chars);
}

// Ex4.08
#[allow(dead_code)]
fn collatz_conjecture() {
    print!("Enter value for nmax: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading user input");
    let nmax = input.trim().parse::<u32>().unwrap();

    let (mut _max_value, mut _max_cycle, mut _start_from): (u32, u32, u32) = (0, 0, 0);
    // an special case handling
    if nmax == 1 {
        println!("start = {:7}, cycles = {:5}, max = {:10}", 1,1,1);
        std::process::exit(0);
    }
    for n_start in (0..=nmax).step_by(1) {
        let (mut _n, mut _cycle_count, mut _temp_max_value): (u32, u32, u32) = (n_start, 0, 0);
        while _n > 1 {
            // find the max value in the cycle
            if _n % 2 == 0 {
                _n = _n / 2;
            } else {
                _n = _n * 3 + 1;
            }

            if _n > _temp_max_value {
                _temp_max_value = _n;
            }
            _cycle_count += 1;
            if _cycle_count > _max_cycle {
                _max_cycle = _cycle_count;
                _start_from = n_start;
                _max_value = _temp_max_value;
            }
        }
    }
    println!("start = {:7}, cycles = {:5}, max = {:10}", _start_from, _max_cycle, _max_value);
}

#[allow(dead_code)]
fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn next_prime(){
    let mut user_input = String::new();
    print!("Enter an integer value: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input).expect("Error reading user input");
    let _n = user_input.trim().parse::<u32>().unwrap();
    let mut _next_prime = _n + 1;
    while !is_prime(_next_prime) {
        _next_prime += 1;
    }
    println!("The next prime is     : {}", _next_prime);
}

#[allow(dead_code)]
fn sum_of_previous_prime() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error reading user input");
    let mut numbers = Vec::new();
    for elements in user_input.split_whitespace() {
        numbers.push(elements.parse::<u32>().unwrap());
    }
    for number in numbers {
        let mut _sum = 0;
        for i in 2..number {
            if is_prime(i) {
                _sum += i;
            }
        }
        if is_prime(_sum) {
            println!("Sum of primes <= {} is {}, which is prime", number, _sum);
        } else {
            println!("Sum of primes <= {} is {}, which is not prime", number, _sum);
        }
    }
}
#[allow(dead_code)]
fn pow(base: u128, exponent: u128) -> u128 {
    let mut _result = 1;
    for _ in 0..exponent {
        _result *= base;
    }
    _result
}

#[allow(dead_code)]
fn cal_log() {
    print!("Enter a number: ");
    std::io::stdout().flush().unwrap();
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("error reading user input");
    let _number = user_input.trim().parse::<u128>().unwrap();
    for x in 2..=_number {
        if pow(2, x) >= _number {
            println!("log2({}) = {}", _number, x);
            break;
        }
    }
}

