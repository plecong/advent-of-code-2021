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

struct BitCriteria {
    default: char,
    take_most: bool,
}

fn filter_rating(mut input: Vec<String>, pos: usize, criteria: &BitCriteria) -> Vec<String> {
    let (zeroes, ones): (Vec<String>, Vec<String>) = input
        .drain(..)
        .partition(|n| n.chars().nth(pos).unwrap() == '0');
    if zeroes.len() > ones.len() {
        if criteria.take_most {
            zeroes
        } else {
            ones
        }
    } else if ones.len() > zeroes.len() {
        if criteria.take_most {
            ones
        } else {
            zeroes
        }
    } else {
        if criteria.default == '0' {
            zeroes
        } else {
            ones
        }
    }
}

fn find_rating(input: Vec<String>, criteria: &BitCriteria) -> isize {
    let mut filtered: Vec<String> = input;
    let mut pos = 0;

    while filtered.len() > 1 {
        filtered = filter_rating(filtered, pos, criteria);
        pos += 1;
    }

    let reading = filtered.get(0).unwrap();
    isize::from_str_radix(&reading, 2).unwrap()
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

fn get_life_support_rating(input: &Vec<String>) -> isize {
    let oxygen_criteria = BitCriteria {
        default: '1',
        take_most: true,
    };
    let oxygen_generator_rating = find_rating(input.to_vec(), &oxygen_criteria);

    let co2_criteria = BitCriteria {
        default: '0',
        take_most: false,
    };
    let co2_scrubber_rating = find_rating(input.to_vec(), &co2_criteria);

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let input = day03_input();
    let bits = count_bits(&input);

    let part1 = get_power_consumtion(&bits);
    println!("Part 1: {}", part1);

    let part2 = get_life_support_rating(&input);
    println!("Part 2: {}", part2);
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

    #[test]
    fn test_filter_rating() {
        let input = sample_input();
        let criteria = BitCriteria {
            default: '1',
            take_most: true,
        };
        let filtered = filter_rating(input, 0, &criteria);
        assert_eq!(filtered.len(), 7);
    }

    #[test]
    fn test_find_oxygen_rating() {
        let input = sample_input();
        let oxygen_criteria = BitCriteria {
            default: '1',
            take_most: true,
        };
        let rating = find_rating(input, &oxygen_criteria);
        assert_eq!(rating, 23);
    }

    #[test]
    fn test_find_co2_rating() {
        let input = sample_input();
        let co2_criteria = BitCriteria {
            default: '0',
            take_most: false,
        };
        let rating = find_rating(input, &co2_criteria);
        assert_eq!(rating, 10);
    }
}
