use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/bin/day1/input1.txt").expect("Reading input file");

    let mut calories = vec![0];
    for val in input.split("\n") {
        if val.is_empty() { 
            calories.push(0);
            continue;
        };
        
        let parsed_val = val.parse::<u32>().unwrap();
        *calories.last_mut().unwrap() += parsed_val;
    }
    calories.sort();
    println!("{:?}", calories.drain(calories.len() - 3..).sum::<u32>());
}
