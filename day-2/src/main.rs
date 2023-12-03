use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*, BufReader};

struct Item {
    color: String,
    score: i32,
}

fn get_item(input: &str) -> Item {
    let mut split_input = input.split_whitespace();
    let num = split_input.next().unwrap();
    let name: &str = split_input.next().unwrap();
    Item {
        color: name.trim().to_string(),
        score: num.parse::<i32>().unwrap(),
    }
}

fn main() -> io::Result<()> {
    let input_file = fs::File::open("/Users/adam.smith/repo/code-advent/day-2/src/input.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(input_file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_iter = line.split(":");
        let _game_str = line_iter.next();

        let data: Vec<Item> = line_iter
            .next()
            .unwrap()
            .split([',', ';'])
            .map(|s| get_item(s.trim()))
            .collect();

        let mut high_score = HashMap::from([
            (String::from("red"), 0),
            (String::from("blue"), 0),
            (String::from("green"), 0),
        ]);
        for item in data {
            if high_score
                .get(&item.color)
                .unwrap()
                < &item.score
            {
                high_score.insert(item.color, item.score);
            }
        }

        total = total + high_score.get("red").unwrap()*high_score.get("blue").unwrap()*high_score.get("green").unwrap();
    }
    println!("Total: {}", total);
    Ok(())
}
