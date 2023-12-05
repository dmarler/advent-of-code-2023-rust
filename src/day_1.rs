
pub fn part_1(input: String) {
    println!("Day 1, Part 1");

    let mut total: u32 = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        // Walk from left
        let line_length = chars.len();
        let mut counter = 0;

        loop {
            if counter >= line_length {
                break;
            }

            let c = chars[counter].to_digit(10);

            match c {
                None => {},
                Some(num) => {
                    total += num * 10;
                    break;
                }
            }

            counter += 1;
        }

        // Walk from right
        counter = line_length - 1;

        loop {

            let c = chars[counter].to_digit(10);

            match c {
                None => {},
                Some(num) => {
                    total += num;
                    break;
                }
            }

            counter -= 1;
        }
    }

    println!("Total: {}", total);
}

pub fn part_2(input: String) {
    println!("Day 1, Part 2");
    let mut converted_input = input;

    let nums = [
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("nineight", "98"),
        ("zerone", "01"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")];

    for (key, value) in nums {
        converted_input = converted_input.replace(key, value);
    }

    part_1(converted_input);
}