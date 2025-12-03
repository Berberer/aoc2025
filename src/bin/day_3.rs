fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_first_max(bank_joltages: &[u8]) -> (usize, &u8) {
    bank_joltages
        .iter()
        .enumerate()
        .reduce(|(current_max_index, current_max), (index, value)| {
            if value > current_max {
                (index, value)
            } else {
                (current_max_index, current_max)
            }
        })
        .unwrap()
}

fn find_maximum_joltage(needed_batteries: usize, bank_joltages: &[u8]) -> Vec<u8> {
    if bank_joltages.len() == 1 {
        Vec::from(bank_joltages)
    } else {
        let (max_joltage_index, max_joltage) =
            // find_first_max(&bank_joltages[0..=(bank_joltages.len() - needed_batteries)]);
            find_first_max(&bank_joltages[0..=(bank_joltages.len() - needed_batteries)]);
        let mut max_joltages = vec![*max_joltage];

        if needed_batteries > 1 {
            max_joltages.append(&mut find_maximum_joltage(
                needed_batteries - 1,
                &bank_joltages[max_joltage_index + 1..],
            ));
        }

        max_joltages
    }
}

fn get_total_output_joltage(needed_batteries: usize, joltages: &[Vec<u8>]) -> u64 {
    joltages
        .iter()
        .map(|bank_joltages| {
            find_maximum_joltage(needed_batteries, bank_joltages)
                .iter()
                .fold(0, |acc, &joltage| acc * 10 + joltage as u64)
        })
        .sum()
}

fn main() {
    let input = include_str!("../inputs/data_day_3.txt");
    let joltages = parse_input(input);

    // Solution for puzzle 1
    let total_output_joltage = get_total_output_joltage(2, &joltages);
    println!("The total output joltage is {total_output_joltage}");

    // Solution for puzzle 2
    let total_output_joltage = get_total_output_joltage(12, &joltages);
    println!("The total output joltage with static friction is {total_output_joltage}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        ";
        assert_eq!(
            parse_input(input),
            vec![
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
            ]
        );
    }

    #[test]
    fn test_find_first_max() {
        assert_eq!(find_first_max(&vec![1]), (0, &1));
        assert_eq!(find_first_max(&vec![1, 2, 3]), (2, &3));
        assert_eq!(find_first_max(&vec![1, 2, 3, 3]), (2, &3));
    }

    #[test]
    fn test_find_maximum_joltage() {
        assert_eq!(
            find_maximum_joltage(2, &vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            vec![9, 8]
        );
        assert_eq!(
            find_maximum_joltage(2, &vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            vec![8, 9]
        );
        assert_eq!(
            find_maximum_joltage(2, &vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            vec![7, 8]
        );
        assert_eq!(
            find_maximum_joltage(2, &vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            vec![9, 2]
        );
    }

    #[test]
    fn test_find_maximum_joltage_with_static_friction() {
        assert_eq!(
            find_maximum_joltage(12, &vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]
        );
        assert_eq!(
            find_maximum_joltage(12, &vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
        );
        assert_eq!(
            find_maximum_joltage(12, &vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        );
        assert_eq!(
            find_maximum_joltage(12, &vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            vec![8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
    }

    #[test]
    fn test_get_total_output_joltage() {
        assert_eq!(
            get_total_output_joltage(
                2,
                &vec![
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                    vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                    vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                    vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
                ]
            ),
            357
        );
    }

    #[test]
    fn test_get_total_output_joltage_with_static_friction() {
        assert_eq!(
            get_total_output_joltage(
                12,
                &vec![
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                    vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                    vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                    vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
                ]
            ),
            3121910778619
        );
    }
}
