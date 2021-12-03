use std::fs;

fn day03_input() -> Vec<String> {
    let contents = fs::read_to_string("./day03/input/input.txt").unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn count_bits(input: &Vec<String>) -> (Vec<u32>, usize) {
    let first = &input[0];
    let counts: Vec<u32> = vec![0; first.len()];
    let counts = input.iter().fold(counts, |acc, x| {
        acc.iter()
            .zip(x.chars())
            .map(|(x, y)| {
                x + match y {
                    '1' => 1,
                    _ => 0,
                }
            })
            .collect()
    });

    (counts, input.len())
}

fn get_epsilon_rate(bits: &(Vec<u32>, usize)) -> isize {
    let (counts, size) = bits;
    let half = (*size as f32) / 2.0;
    let epsilon_str: String = counts
        .iter()
        .map(|&x| if (x as f32) < half { '1' } else { '0' })
        .collect();

    isize::from_str_radix(&epsilon_str, 2).unwrap()
}

fn get_gamma_rate(bits: &(Vec<u32>, usize)) -> isize {
    let (counts, size) = bits;
    let half = (*size as f32) / 2.0;
    let gamma_str: String = counts
        .iter()
        .map(|&x| if x as f32 > half { '1' } else { '0' })
        .collect();

    isize::from_str_radix(&gamma_str, 2).unwrap()
}

fn get_power_consumtion(bits: &(Vec<u32>, usize)) -> isize {
    let gamma = get_gamma_rate(bits);
    let epsilon = get_epsilon_rate(bits);
    gamma * epsilon
}

fn main() {
    let input = day03_input();
    let bits = count_bits(&input);

    let part1 = get_power_consumtion(&bits);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<String> {
        let sample = "00100\n\
            11110\n\
            10110\n\
            10111\n\
            10101\n\
            01111\n\
            00111\n\
            11100\n\
            10000\n\
            11001\n\
            00010\n\
            01010";

        sample.lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_count() {
        let input = sample_input();
        let bits = count_bits(&input);
        assert_eq!(bits.1, 12)
    }

    #[test]
    fn test_gamma_rate() {
        let input = sample_input();
        let bits = count_bits(&input);
        let gamma = get_gamma_rate(&bits);
        assert_eq!(gamma, 22);
    }

    #[test]
    fn test_epsilon_rate() {
        let input = sample_input();
        let bits = count_bits(&input);
        let epsilon = get_epsilon_rate(&bits);
        assert_eq!(epsilon, 9);
    }
}
