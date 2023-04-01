// Ex3.04 _date Tomorrow

// HACK: if you are sure all varaibles will be used later, then please add _ before the variable name
// like _year, _month, _date, _month_date, leap_year here

#[allow(dead_code)]
pub fn date_tomorrow_with_year() {
    let (mut _year, mut _month, mut _date, mut _month_date, mut leap_year):(i32, i32, i32, i32, bool) = (0, 0, 0, 0, false);
    println!("Enter a _date in dd/mm/yyyy format: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let parts: Vec<_> = input
        .trim()
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    _date = parts[0];
    _month = parts[1];
    _year = parts[2];

    // Check if the _year is a leap _year
    if (_year & 3) == 0 && ((_year % 25) != 0 || (_year & 15) == 0) {
      leap_year = true;
    }

    // Check if the _date is the last day of the _month
    if _month == 1 || _month == 3 || _month == 5 || _month == 7 || _month == 8 || _month == 10 || _month == 12 {
      _month_date = 31;
    } else if _month == 4 || _month == 6 || _month == 9 || _month == 11 {
      _month_date = 30;
    } else if _month == 2 {
      if leap_year == true {
        _month_date = 29;
      } else {
        _month_date = 28;
      }
    }

    // output today's _date
    println!("Today's _date is {}/{}/{}", _date, _month, _year);

    // calculate tommoorow's _date
    if _month == 12 && _date + 1 > _month_date {
        _date = 1;
        _month = 1;
        _year+=1;
    } else if _date + 1 > _month_date {
        _month = _month + 1;
        _date = 01;
    } else {
        _date = _date + 1;
    }

    // output result
    println!("Tomorrow's _date is {}/{}/{}", _date, _month, _year);

}

// Ex3.05 Temperature Conversion
#[allow(dead_code)]
pub fn temperature_conversion() {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let mut iter = user_input.trim().chars();
    let mut temperature = String::new();
    let mut unit = '\0';
    // Start a infinite loop, until the user input a non-digit character
    loop {
        // match the iterator, it will keep track of next character
        // and return c if there is a character, else it will return None
        match iter.next() {
            // Some(c) means the iterator returns a character
            Some(c) => {
                // check if this character is a digit or a dot or a minus
                // as dot or minus can be an connector of two numbers
                // this .is_digit(10) is a method of char type
                // it will check if this character is a digit in _base 10
                // if it is, then it will return true
                if c.is_digit(10) || c == '.' || c == '-' {
                    // if it is, then add it to the temperature string
                    temperature.push(c);
                } else {
                    // else, this is a unit, so we break the loop
                    // but we need to check if this is a valid unit, like C, F, K
                    // if it is not, then we panic and throw an error and abort
                    if c != 'C' && c != 'F' && c != 'K' {
                        panic!("Invalid unit");
                    } else {
                        unit = c;
                    }
                    break;
                }
            }
            // break when the iterator returns None
            None => break,
        }
    }
    // generally it should be safe to unwrap the temperature String
    // because we have checked if it is a valid numbers
    let temperature: f64 = temperature.parse().expect("Failed to parse temperature");
    println!("Temperature: {}{}", temperature, unit);
}


// Ex3.07b: Generalised converter

// use std::io;
use std::io::{self, Write};
#[allow(dead_code)]
pub fn generalised_converter() {
    let mut user_input = String::new();
    print!("Enter a quntitiy: ");
    // refresh print! buffer
    io::stdout().flush().expect("Failed to flush stdout");
    // read user input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading input");
    // define varaibles to store quantity and unit
    let mut iter = user_input.trim().chars();
    let mut unit = String::new();
    let mut quantity = String::new();
    // sepa_rate quantity and unit from user user input with infinite loop
    // HACK: there's a cargo package `scan_fmt` that can do this in a more elegant way
    loop {
        match iter.next() {
            Some(c) => {
                if c.is_digit(10) || c == '.' || c == '-' {
                    quantity.push(c);
                } else {
                    if c != 'C' && c != 'F' && c != 'M' && c != 'K' && c != 'P' && c != 'G' {
                    // NOTE: Why `||` will cause panic while `&&` wont?
                    // In the original code, the if statement that checks for invalid unit 
                    // characters uses the || (OR) operator, which means that it will evaluate 
                    // to true if the character is not ‘C’, OR the character is not ‘F’, OR…etc. 
                    // In other words, the if statement will always evaluate to true since 
                    // the character can’t be all of the valid unit characters at the same time.
                    // if c != 'C' || c != 'F' || c != 'M' || c != 'K' || c != 'P' || c != 'G' {
                        panic!("Invalid unit, c = {c}");
                    } else {
                        unit.push(c);
                    }
                    break;
                }
            }
            None => break,
        }
    }
    // convert quantity to f64
    let quantity: f64 = quantity.parse().expect("Failed to parse quantity");
    // check the unit, and convert to the target unit
    if unit == "C" {
        println!("{}C = {}F", quantity, quantity * 9.0 / 5.0 + 32.0);
    } else if unit == "F" {
        println!("{}F = {}C", quantity, (quantity - 32.0) * 5.0 / 9.0);
    } else if unit == "M" {
        println!("{}M = {}K", quantity, quantity * 1.609);
    } else if unit == "K" {
        println!("{}K = {}M", quantity, quantity / 1.609);
    } else if unit == "P" {
        println!("{}P = {}G", quantity, quantity * 454.0);
    } else if unit == "G" {
        println!("{}G = {}P", quantity, quantity / 454.0);
    }
}

// Ex3.08 Calculating additional taxes

#[allow(dead_code)]
pub fn additional_taxes() {
    // define different tax _rate
    let _rate0 = 0.000;
    let _rate1 = 0.190;
    let _rate2 = 0.325;
    let _rate3 = 0.370;
    let _rate4 = 0.450;
    let _rate5 = 0.475;
    // define medicare _rate
    let _ratem0 = 0.015;
    let _ratem1 = 0.025;

    // defien _thresh
    let _thresh1 = 18200.00;
    let _thresh2 = 37000.00;
    let _thresh3 = 80000.00;
    let _thresh4 = 180000.00;
    let _thresh5 = 250000.00;
    let _thresh6 = 100000.00;

    // calculate _base tax
    let _base0 = 0.00;
    let _base1 = _base0 + _rate0 * _thresh1;
    let _base2 = _base1 + _rate1 * (_thresh2 - _thresh1);
    let _base3 = _base2 + _rate2 * (_thresh3 - _thresh2);
    let _base4 = _base3 + _rate3 * (_thresh4 - _thresh3);


    // define variables
    let mut user_input = String::new();
    let (mut _gross, mut _tax, mut _medicare, mut _net) = (0.00,0.00,0.00,0.00);
    // get user's _gross salary 
    print!("Enter gross salary: $");
    // refresh print! buffer
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input")
        .to_string();
    // phrase into f64
    let _gross: f64 = user_input.trim().parse::<f64>().unwrap();
    // calculate _tax fee
    if _gross < _thresh1 {
        _tax = _base0 + _gross * _rate0;
    } else if _gross <= _thresh2 {
        _tax = _base1 + (_gross - _thresh1) * _rate1;
    } else if _gross <= _thresh3 {
        _tax = _base2 + (_gross - _thresh2) * _rate2;
    } else if _gross <= _thresh4 {
        _tax = _base3 + (_gross - _thresh3) * _rate3;
    } else {
        if _gross <= _thresh5 {
            _tax = _base4 + (_gross - _thresh4) * _rate4;
        } else {
            _tax = _base4 + (_thresh5 - _thresh4) * _rate4 + (_gross - _thresh5) * _rate5;
        }
    }

    // calculate medicare fee
    if _gross <= _thresh6 {
        _medicare = _gross * _ratem0;
    } else {
        _medicare = _gross * _ratem1;
    }
    // print out _gross salary
    // NOTE: {:>9.2} means right align the number with at least 9 char length
    // 9 is a sum of both digit before and after the decimal point
    println!("gross incoe       : ${:>9.2}", _gross);
    // print out _tax fee
    println!("less tax          : ${:>9.2}", -_tax);
    // print out medicare fee
    println!("less medicare levy: ${:>9.2}", -_medicare);
    // print out net income
    println!("net income        : ${:>9.2}", _gross - _tax - _medicare);
}

