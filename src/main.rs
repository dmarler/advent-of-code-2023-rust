use std::fs;

mod day_1;
mod day_2;

fn main() {


    let file: String = fs::read_to_string("./src/inputs/day_2.txt").expect("Failed to load file.");

    day_2::part_1(file);

}