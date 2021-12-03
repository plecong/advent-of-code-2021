use std::fs;

fn day01_input(contents: &str) -> Vec<i32> {
    contents
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./day01/input/input1.txt").expect("Error reading input");
    let input = day01_input(&contents);

    let count = input
        .windows(2)
        .map(|pair| pair[1] - pair[0] > 0)
        .filter(|&x| x)
        .count();

    println!("{}", count)
}
