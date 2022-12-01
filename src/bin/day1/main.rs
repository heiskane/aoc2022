use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let input = read_to_string("src/bin/day1/input1.txt");

    let mut calories: HashMap<u32, u32> = HashMap::new();
    let mut elf_count = 1;
    for val in input.unwrap().split("\n") {
        if val.is_empty() { 
            elf_count +=1;
            continue;
        };
        
        let parsed_val = val.parse::<u32>().unwrap();
        if !calories.contains_key(&elf_count) {
            calories.insert(elf_count, parsed_val);
        } else {
            *calories.get_mut(&elf_count).unwrap() += parsed_val;
        }

    }
    let max_val = calories.iter().max_by_key(|e| e.1).unwrap();
    println!("{:?}", max_val);
}
