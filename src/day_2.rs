
pub fn part_1(input: String) -> u32 {
    println!("Day 2, Part 1");

    let mut total: u32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(":")
            .collect();

        let game: i8 = parts[0]
            .split(" ")
            .collect::<Vec<_>>()[1]
            .parse()
            .expect("Failed to convert game number");

        println!("{game}");
    }

    return total;
}