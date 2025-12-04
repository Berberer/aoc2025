fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect()
}

fn check_cell(x: i32, y: i32, map: &[Vec<bool>]) -> Option<bool> {
    if y < 0 || y >= map.len() as i32 || x < 0 || x >= map[y as usize].len() as i32 {
        None
    } else {
        Some(map[y as usize][x as usize])
    }
}

fn get_neighborhood(x: i32, y: i32, map: &[Vec<bool>]) -> [Option<bool>; 8] {
    [
        check_cell(x - 1, y - 1, map),
        check_cell(x, y - 1, map),
        check_cell(x + 1, y - 1, map),
        check_cell(x - 1, y, map),
        check_cell(x + 1, y, map),
        check_cell(x - 1, y + 1, map),
        check_cell(x, y + 1, map),
        check_cell(x + 1, y + 1, map),
    ]
}

fn is_accessible(neighborhood: [Option<bool>; 8]) -> bool {
    neighborhood.iter().filter(|c| **c == Some(true)).count() < 4
}

fn find_accessible_rolls(map: &[Vec<bool>]) -> Option<Vec<(usize, usize)>> {
    let mut coordinates = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] && is_accessible(get_neighborhood(x as i32, y as i32, map)) {
                coordinates.push((x, y));
            }
        }
    }

    if coordinates.is_empty() {
        None
    } else {
        Some(coordinates)
    }
}

fn clear_up_map(map: &mut [Vec<bool>]) -> Vec<Vec<(usize, usize)>> {
    let mut removed_rolls = Vec::new();

    while let Some(accessible_rolls) = find_accessible_rolls(map) {
        for (x, y) in &accessible_rolls {
            map[*y][*x] = false;
        }

        removed_rolls.push(accessible_rolls);
    }

    removed_rolls
}

fn main() {
    let input = include_str!("../inputs/data_day_4.txt");
    let mut map = parse_input(input);

    // Solution for puzzle 1
    let accessible_rolls = find_accessible_rolls(&map);
    println!(
        "{} rolls are immediately accessible via forklift",
        accessible_rolls.unwrap().len()
    );

    // Solution for puzzle 2
    let removed_rolls = clear_up_map(&mut map);
    let overall_removed_rolls = removed_rolls.iter().map(Vec::len).sum::<usize>();
    println!("{overall_removed_rolls} can be removed overall via forklift");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            ..@@
            @@@.
        ";
        assert_eq!(
            parse_input(input),
            vec![
                vec![false, false, true, true],
                vec![true, true, true, false]
            ]
        );
    }

    #[test]
    fn test_get_neighbor() {
        let map = vec![
            vec![false, false, true, true],
            vec![true, true, true, false],
        ];

        assert_eq!(
            get_neighborhood(0, 0, &map),
            [
                None,
                None,
                None,
                None,
                Some(false),
                None,
                Some(true),
                Some(true)
            ]
        );
        assert_eq!(
            get_neighborhood(3, 1, &map),
            [
                Some(true),
                Some(true),
                None,
                Some(true),
                None,
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn test_find_accessible_rolls() {
        let map = vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];
        assert_eq!(find_accessible_rolls(&map), None);

        let map = vec![vec![false, true, true, true], vec![true, true, true, false]];
        assert_eq!(find_accessible_rolls(&map), Some(vec![(3, 0), (0, 1)]));
    }
}
