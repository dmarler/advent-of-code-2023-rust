use std::fs;

mod day_1;

fn main() {


    let file: String = fs::read_to_string("./src/inputs/day_1.txt").expect("Failed to load file.");

    day_1::part_2(file);

}