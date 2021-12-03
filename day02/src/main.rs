use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
    Unknown,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    count: i32,
}

#[derive(Debug)]
struct Location {
    depth: i32,
    position: i32,
}

impl Location {
    fn result(&self) -> i32 {
        self.depth * self.position
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(forward|down|up) (\d+)").unwrap();
}

fn day02_input() -> Vec<Command> {
    let contents = fs::read_to_string("./day02/input/input.txt").unwrap();
    contents.lines().filter_map(parse_command).collect()
}

fn parse_command(text: &str) -> Option<Command> {
    let caps = RE.captures(text);

    match caps {
        Some(c) => Some(Command {
            direction: match c[1].as_ref() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Unknown,
            },
            count: c[2].parse().unwrap(),
        }),
        None => None,
    }
}

fn get_location(input: &Vec<Command>) -> Location {
    input.iter().fold(
        Location {
            depth: 0,
            position: 0,
        },
        |acc, x| Location {
            depth: match x.direction {
                Direction::Down => acc.depth + x.count,
                Direction::Up => acc.depth - x.count,
                _ => acc.depth,
            },
            position: match x.direction {
                Direction::Forward => acc.position + x.count,
                _ => acc.position,
            },
        },
    )
}

fn main() {
    let input = day02_input();
    let location = get_location(&input);
    println!("Part 1: {}", location.result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let command = parse_command("forward 5").unwrap();
        assert_eq!(command.direction, Direction::Forward);
        assert_eq!(command.count, 5);
    }

    #[test]
    fn test_position() {
        let sample = "forward 5\n\
            down 5\n\
            forward 8\n\
            up 3\n\
            down 8\n\
            forward 2";
        let input = sample.lines().filter_map(parse_command).collect();
        let location = get_location(&input);
        assert_eq!(location.depth, 10);
        assert_eq!(location.position, 15);
        assert_eq!(location.result(), 150);
    }
}
