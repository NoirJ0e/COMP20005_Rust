// Ex2.08
use std::io as io;

#[allow(dead_code)]
pub fn c02() {
    let mut user_input;
    user_input = String::new();
    // std::io::stdin().read_line(&mut user_input).expect("Failed to read input!");
    // NOTE: if i keep `use std::io` it will make the compilor confused
    println!("Enter degrees F:");
    // HACK: Rust has a weired cache of output, if i don't use println() i need to manually refresh
    // to make it work,  like io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read input!");
    let degree_in_f: f32 = user_input.trim().parse::<f32>().expect("Failed to parse input!");
    let degree_in_c: f32 = (degree_in_f - 32.0) * (5.0/9.0);
    println!("In degrees C is : {:.2}", degree_in_c); // {:.2} means 2 decimal places
}
