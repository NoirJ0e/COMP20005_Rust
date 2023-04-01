use std::io::Write;
#[allow(dead_code)]
pub fn max_2_ints(a: i128, b: i128) -> i128 {
    if a > b{
        a
    } else {
        b
    }
}

#[allow(dead_code)]
pub fn max_4_ints(a: i128, b: i128, c: i128, d: i128) -> i128 {
    // we can use max_2_ints
    max_2_ints(max_2_ints(a, b), max_2_ints(c, d))
}

#[allow(dead_code)]
pub fn median_3_ints(a: i128, b: i128, c:i128) -> i128 {
    // we can use max_2_ints
    let temp = max_2_ints(a, b);
    if max_2_ints(temp, c) == temp {
        max_2_ints(a, b)
    } else {
        max_2_ints(b, c)
    }
}

#[allow(dead_code)]
pub fn int_pows(base: i128, pow: i128) -> i128 {
    let mut result = 1;
    for _ in 0..pow {
        result *= base;
    }
    result
}

#[allow(dead_code)]
fn factroial(base: i128) -> i128 {
    let mut result = 1;
    for i in 1..base+1 {
        result *= i;
    }
    result
}

#[allow(dead_code)]
pub fn n_choose_k(n: i128, k: i128) -> i128 {
    // a simplification method to calculate factroial division
    let part = n - k + 1;
    let mut result: i128 = 1;
    for i in part..n+1 {
        result *= i;
    }
    result / factroial(k)
}

#[allow(dead_code)]
pub fn amicable_pair(a: i128, b: i128) -> bool {
    // sum of divsors of a
    let mut sum_a: i128 = 0;
    for i in 1..a {
        if a % i == 0 {
            sum_a += i;
        }
    }

    // sum of divsors of b 
    let mut sum_b: i128 = 0;
    for i in 1..b {
        if b % i == 0 {
            sum_b += i;
        }
    }
    if sum_a == b && sum_b == a {
        true
    } else {
        false
    }
}

// this is a implementation of factroial but in f64, normallly it won't be float number
// however, this time the program require a f64 type return and i cant figure out how to transform
// the type without writing a new function, so this is it
#[allow(dead_code)]
fn factroial_f64(n: f64) -> f64 {
    (1..=(n as u64)).map(|x| x as f64).product()
}



#[allow(dead_code)]
fn sum_of_n(n: i128) -> f64 {
    let mut result = n as f64;
    let mut count = n - 1;
    while count > 0 {
        result += count as f64;
        count -= 1;
    }
    result
}
#[allow(dead_code)]
pub fn sum_of_sequence(n: i128) -> f64 {
    let mut result = 0.0;
    let mut loop_count = 1;
    while loop_count <= n {
        // HACK: This is how we do it without rewriting the factroial function
        // this is the way while rewriting the factroial function
        // result += sum_of_n(loop_count) / factroial_f64(loop_count as f64);
        result += sum_of_n(loop_count) / factroial(loop_count) as f64;
        loop_count += 1;
    }
    result
}
