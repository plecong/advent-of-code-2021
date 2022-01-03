use std::fs;

#[derive(Debug)]
struct BingoInput {
    draw: String,
    boards: Vec<BingoBoard>,
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<i32>,
}

impl BingoBoard {
    fn new(values: &[String]) -> Self {
        BingoBoard {
            board: values
                .iter()
                .map(|s| s.split_whitespace())
                .flatten()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

fn day04_input() -> Vec<String> {
    let contents = fs::read_to_string("./day04/input/sample.txt").unwrap();
    contents.lines().map(|l| l.to_string()).collect()
}

fn parse_input(input: &[String]) -> BingoInput {
    let boards = &input[2..input.len()];
    let chunks = boards.chunks(6);

    BingoInput {
        draw: input[0].to_string(),
        boards: chunks.map(|c| BingoBoard::new(c)).collect(),
    }
}

fn main() {
    let input = day04_input();
    let bingo_input = parse_input(&input);

    println!("{:?}", bingo_input);
}
