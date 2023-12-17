use std::cmp::max;
use std::collections::HashMap;

pub fn part_1(input: String) -> i32 {
    println!("Day 2, Part 1");

    let mut total: i32 = 0;

    for line in input.lines() {
        let (game, mut scores) = get_game_maxes(line);

        // Check scores
        if *scores.entry("red").or_default() <= 12
            && *scores.entry("green").or_default() <= 13
            && *scores.entry("blue").or_default() <= 14
        {
            total += game;
        }

        scores.clear();
    }

    return total;
}

pub fn part_2(input: String) -> i32 {
    let mut power = 0;

    for line in input.lines() {
        let (_, mut scores) = get_game_maxes(line);
        power += *scores.entry("red").or_default()
            * *scores.entry("blue").or_default()
            * *scores.entry("green").or_default();
    }

    return power;
}

fn get_game_maxes(line: &str) -> (i32, HashMap<&str, i32>) {
    let parts: Vec<&str> = line.split(":").collect();

    let game: i32 = parts[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .expect("Failed to convert game number");

    let rounds = parts[1].split(";").collect::<Vec<&str>>();

    let mut scores: HashMap<&str, i32> = HashMap::new();

    // For each round:
    for round in rounds {
        let colors = round.split(",").collect::<Vec<&str>>();

        for color in colors {
            let values: Vec<&str> = color.trim().split(" ").collect::<Vec<&str>>();

            let current: i32 = *scores.entry(values[1]).or_default();

            scores.insert(
                values[1],
                max(
                    values[0].parse::<i32>().expect("Failed to parse dice."),
                    current,
                ),
            );
        }
    }
    (game, scores)
}
