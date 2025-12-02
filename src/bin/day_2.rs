use std::iter;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|id_range| id_range.split_once('-').unwrap())
        .map(|(left_id, right_id)| {
            (
                left_id.parse::<u64>().unwrap(),
                right_id.parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn is_invalid_id_halves(id: &u64) -> bool {
    let id = id.to_string();
    if id.len().is_multiple_of(2) {
        let (first_half, second_half) = id.split_at(id.len() / 2);
        first_half == second_half
    } else {
        false
    }
}

fn find_invalid_ids_halves((left_id, right_id): &(u64, u64)) -> Vec<u64> {
    (*left_id..=*right_id)
        .filter(is_invalid_id_halves)
        .collect()
}

fn is_invalid_id_sequence(sequence: &str, id: &str) -> bool {
    iter::repeat_n(sequence, id.len() / sequence.len()).collect::<String>() == id
}

fn is_invalid_id_any_length(id: &u64) -> bool {
    let id = id.to_string();

    for sequence_length in 1..=(id.len() / 2) {
        if is_invalid_id_sequence(&id[0..sequence_length], &id) {
            return true;
        }
    }

    false
}

fn find_invalid_ids_any_length((left_id, right_id): &(u64, u64)) -> Vec<u64> {
    (*left_id..=*right_id)
        .filter(is_invalid_id_any_length)
        .collect()
}

fn main() {
    let input = include_str!("../inputs/data_day_2.txt");
    let id_ranges = parse_input(input);

    // Solution for puzzle 1
    let invalid_ids = id_ranges
        .iter()
        .flat_map(find_invalid_ids_halves)
        .collect::<Vec<u64>>();
    let invalid_ids_sum = invalid_ids.iter().sum::<u64>();
    println!("The sum of invalid ids with two repeating halves is {invalid_ids_sum}");

    // Solution for puzzle 2
    let invalid_ids = id_ranges
        .iter()
        .flat_map(find_invalid_ids_any_length)
        .collect::<Vec<u64>>();
    let invalid_ids_sum = invalid_ids.iter().sum::<u64>();
    println!("The sum of invalid ids with repeating sequences is {invalid_ids_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(
            parse_input(input),
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124),
            ]
        );
    }

    #[test]
    fn test_is_invalid_id_halves() {
        assert_eq!(is_invalid_id_halves(&998), false);
        assert_eq!(is_invalid_id_halves(&1010), true);
        assert_eq!(is_invalid_id_halves(&1012), false);
        assert_eq!(is_invalid_id_halves(&101010), false);
    }

    #[test]
    fn test_find_invalid_ids_halves() {
        assert_eq!(find_invalid_ids_halves(&(11, 22)), vec![11, 22]);
        assert_eq!(find_invalid_ids_halves(&(95, 115)), vec![99]);
        assert_eq!(find_invalid_ids_halves(&(998, 1012)), vec![1010]);
        assert_eq!(
            find_invalid_ids_halves(&(1188511880, 1188511890)),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids_halves(&(222220, 222224)), vec![222222]);
        assert_eq!(find_invalid_ids_halves(&(1698522, 1698528)), vec![]);
        assert_eq!(find_invalid_ids_halves(&(446443, 446449)), vec![446446]);
        assert_eq!(
            find_invalid_ids_halves(&(38593856, 38593862)),
            vec![38593859]
        );
        assert_eq!(find_invalid_ids_halves(&(565653, 565659)), vec![]);
        assert_eq!(find_invalid_ids_halves(&(824824821, 824824827)), vec![]);
        assert_eq!(find_invalid_ids_halves(&(2121212118, 2121212124)), vec![]);
    }

    #[test]
    fn test_is_invalid_id_any_length() {
        assert_eq!(is_invalid_id_any_length(&998), false);
        assert_eq!(is_invalid_id_any_length(&1010), true);
        assert_eq!(is_invalid_id_any_length(&1012), false);
        assert_eq!(is_invalid_id_any_length(&101010), true);
    }

    #[test]
    fn test_find_invalid_ids_any_length() {
        assert_eq!(find_invalid_ids_any_length(&(11, 22)), vec![11, 22]);
        assert_eq!(find_invalid_ids_any_length(&(95, 115)), vec![99, 111]);
        assert_eq!(find_invalid_ids_any_length(&(998, 1012)), vec![999, 1010]);
        assert_eq!(
            find_invalid_ids_any_length(&(1188511880, 1188511890)),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids_any_length(&(222220, 222224)), vec![222222]);
        assert_eq!(find_invalid_ids_any_length(&(1698522, 1698528)), vec![]);
        assert_eq!(find_invalid_ids_any_length(&(446443, 446449)), vec![446446]);
        assert_eq!(
            find_invalid_ids_any_length(&(38593856, 38593862)),
            vec![38593859]
        );
        assert_eq!(find_invalid_ids_any_length(&(565653, 565659)), vec![565656]);
        assert_eq!(
            find_invalid_ids_any_length(&(824824821, 824824827)),
            vec![824824824]
        );
        assert_eq!(
            find_invalid_ids_any_length(&(2121212118, 2121212124)),
            vec![2121212121]
        );
    }
}
