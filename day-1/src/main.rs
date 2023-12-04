use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

fn get_num(input: String) -> Result<i32, std::num::ParseIntError> {
    let numbers: Vec<String> = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    for (i, my_char) in input.chars().enumerate() {
        if my_char.is_numeric() {
            if first_char.is_none() {
                first_char = Some(my_char);
            }
            last_char = Some(my_char);
        }
        for number in &numbers {
            if number.starts_with(my_char) {
                if input.len() >= i + number.len() {
                    if compare_string(number, &input[i..i + number.len()]) {
                        if first_char.is_none() {
                            first_char = string_to_i32(number);
                        }
                        last_char = string_to_i32(number);
                    }
                }
            }
        }
    }
    let char_array = [first_char.unwrap(), last_char.unwrap()];
    println!("first char: {}, second char: {}", char_array[0], char_array[1] );
    let string_val = String::from_iter(char_array);
    string_val.parse::<i32>()
}

fn compare_string(string_1: &String, string_2: &str) -> bool {
    println!("1: {}, 2: {}", string_1, string_2);
    return string_1 == string_2;
}

fn string_to_i32(number_string: &str) -> Option<char> {
    let number_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    number_map
        .get(number_string)
        .and_then(|f| Some(f.to_owned()))
}

fn main() {
    let input_file = fs::File::open("/Users/adam.smith/repo/code-advent/day-1/src/input.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(input_file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        // get number
        let num = get_num(line).unwrap();
        println!(" returned number: {}", num);
        total = total + num;
    }
    println!("Total: {}", total);
}
