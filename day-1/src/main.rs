use std::fs;
use std::io::{prelude::*, BufReader};

fn get_num(input: String) -> Result<i32, std::num::ParseIntError> {
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    for my_char in input.chars() {
        if my_char.is_numeric() {
            if first_char.is_none() {
                first_char = Some(my_char);
            }
            last_char = Some(my_char);
        }
    }
    let char_array = [first_char.unwrap(), last_char.unwrap()];
    let string_val = String::from_iter(char_array);
    string_val.parse::<i32>()
}

fn main() {
    let input_file = fs::File::open("/Users/adam.smith/repo/code-advent/day-1/src/input.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(input_file);
    let mut total = 0;
    for line in reader.lines() {
        // get number
        let num = get_num(line.unwrap()).unwrap();
        println!("{}", num);
        total = total + num;
    }
    println!("Total: {}", total);
}
