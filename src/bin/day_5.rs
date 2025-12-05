use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (fresh_ingridients_input, available_ingridients_input) =
        input.trim().split_once("\n\n").unwrap();

    let fresh_ingridients = fresh_ingridients_input
        .lines()
        .map(|line| line.trim().split_once("-").unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .collect();
    let available_ingridients = available_ingridients_input
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();

    (fresh_ingridients, available_ingridients)
}

fn find_fresh_available_ingridients<'a>(
    fresh_ingridients: &[RangeInclusive<u64>],
    available_ingridients: &'a [u64],
) -> Vec<&'a u64> {
    available_ingridients
        .iter()
        .filter(|ingridient| {
            fresh_ingridients
                .iter()
                .any(|range| range.contains(ingridient))
        })
        .collect()
}

fn merge_ingridient_ranges(
    ingridient_ranges: Vec<RangeInclusive<u64>>,
) -> Vec<RangeInclusive<u64>> {
    let mut ingridient_ranges = ingridient_ranges.clone();
    ingridient_ranges.sort_by_key(|range| *range.start());

    let mut i = 0;
    while i < (ingridient_ranges.len() - 1) {
        if ingridient_ranges[i].contains(ingridient_ranges[i + 1].start())
            || ingridient_ranges[i + 1].contains(ingridient_ranges[i].end())
        {
            let next_range = ingridient_ranges.remove(i + 1);
            ingridient_ranges[i] =
                *ingridient_ranges[i].start()..=*ingridient_ranges[i].end().max(next_range.end());
        } else {
            i += 1;
        }
    }

    ingridient_ranges
}

fn main() {
    let input = include_str!("../inputs/data_day_5.txt");
    let (fresh_ingridients, available_ingridients) = parse_input(input);

    // Solution for puzzle 1
    let fresh_available_ingridients =
        find_fresh_available_ingridients(&fresh_ingridients, &available_ingridients);
    println!(
        "{} of the available ingridients are fresh",
        fresh_available_ingridients.len()
    );

    // Solution for puzzle 2
    let merged_fresh_ingridient_ranges = merge_ingridient_ranges(fresh_ingridients);
    let fresh_ingridients_amount = merged_fresh_ingridient_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>();
    println!("{fresh_ingridients_amount} ingridients are considered to be fresh",);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        ";
        let (fresh_ingridients, available_ingridients) = parse_input(input);

        assert_eq!(fresh_ingridients, vec![3..=5, 10..=14, 16..=20, 12..=18,]);
        assert_eq!(available_ingridients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_find_fresh_available_ingridients() {
        let fresh_ingridients = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let available_ingridients = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(
            find_fresh_available_ingridients(&fresh_ingridients, &available_ingridients),
            vec![&5, &11, &17]
        );
    }

    #[test]
    fn test_merge_ingridient_ranges() {
        let fresh_ingridient_ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

        assert_eq!(
            merge_ingridient_ranges(fresh_ingridient_ranges),
            vec![3..=5, 10..=20]
        );
    }
}
