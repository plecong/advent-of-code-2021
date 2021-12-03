use std::fs;

fn day01_input() -> Vec<i32> {
    let contents = fs::read_to_string("./day01/input/input1.txt").unwrap();
    contents
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect()
}

fn main() {
    let input = day01_input();

    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn increase_count(measures: &Vec<i32>) -> usize {
    measures
        .windows(2)
        .map(|pair| pair[1] - pair[0] > 0)
        .filter(|&x| x)
        .count()
}

fn part1(input: &Vec<i32>) -> usize {
    increase_count(&input)
}

fn part2(input: &Vec<i32>) -> usize {
    let measures: Vec<i32> = input
        .windows(3)
        .map(|triplet| triplet[0] + triplet[1] + triplet[2])
        .collect();

    increase_count(&measures)
}
