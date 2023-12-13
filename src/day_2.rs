use std::collections::HashMap;
use std::cmp::max;

pub fn part_1(input: String) -> i32 {
    println!("Day 2, Part 1");

    let mut total: i32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(":")
            .collect();

        let game: i32 = parts[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse()
            .expect("Failed to convert game number");

        let rounds = parts[1]
            .split(";")
            .collect::<Vec<&str>>();

        let mut scores: HashMap<&str, i32> = HashMap::new();

        // For each round:
        for round in rounds {
            let colors = round
                .split(",")
                .collect::<Vec<&str>>();

            for color in colors {
                let values: Vec<&str> = color
                    .trim()
                    .split(" ")
                    .collect::<Vec<&str>>();

                let current: i32 = *scores.entry(values[1]).or_default();

                scores.insert(
                    values[1],
                    max(values[0]
                        .parse::<i32>()
                        .expect("Failed to parse dice."),
                        current));
            }

        }

        // Check scores
        if
            *scores.entry("red").or_default() <= 12 &&
            *scores.entry("green").or_default() <= 13 &&
            *scores.entry("blue").or_default() <= 14
            {
            total += game;
        }

        scores.clear();

    }

    return total;
}