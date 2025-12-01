#[derive(Debug, PartialEq)]
enum DialRotations {
    Left(i32),
    Right(i32),
}

impl DialRotations {
    fn new(line: &str) -> Self {
        let (direction, amount) = line.trim().split_at(1);
        let amount = amount.parse::<i32>().unwrap();

        if direction == "L" {
            DialRotations::Left(amount)
        } else {
            DialRotations::Right(amount)
        }
    }

    fn apply(&self, position: i32) -> (i32, i32) {
        match self {
            DialRotations::Left(rotation) => {
                let new_position = position - rotation;
                let zero_positions = new_position.abs() / 100;
                let new_position = new_position % 100;

                if new_position < 0 {
                    if position == 0 {
                        (100 + new_position, zero_positions)
                    } else {
                        (100 + new_position, zero_positions + 1)
                    }
                } else if new_position == 0 {
                    (new_position, zero_positions + 1)
                } else {
                    (new_position, zero_positions)
                }
            }
            DialRotations::Right(rotation) => {
                let new_position = position + rotation;
                let zero_positions = new_position / 100;
                let new_position = new_position % 100;

                (new_position, zero_positions)
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<DialRotations> {
    input.trim().lines().map(DialRotations::new).collect()
}

fn exectute_dial_rotation(start: i32, rotations: &[DialRotations]) -> Vec<(i32, i32)> {
    let mut positions = vec![(start, 0)];

    for rotation in rotations {
        positions.push(rotation.apply(positions.last().unwrap().0));
    }

    positions
}

fn generate_password(dial_positions: &[(i32, i32)]) -> usize {
    dial_positions.iter().filter(|(r, _)| *r == 0).count()
}

#[allow(non_snake_case)]
fn generate_password_0x434C49434B(dial_positions: &[(i32, i32)]) -> i32 {
    dial_positions.iter().map(|(_, z)| z).sum()
}

fn main() {
    let input = include_str!("../inputs/data_day_1.txt");
    let dial_rotations = parse_input(input);
    let dial_positions = exectute_dial_rotation(50, &dial_rotations);

    // Solution for puzzle 1
    let password = generate_password(&dial_positions);
    println!("The password is {password}");

    // Solution for puzzle 2
    let password = generate_password_0x434C49434B(&dial_positions);
    println!("The password with method 0x434C49434B is {password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        ";

        assert_eq!(
            parse_input(input),
            vec![
                DialRotations::Left(68),
                DialRotations::Left(30),
                DialRotations::Right(48),
                DialRotations::Left(5),
                DialRotations::Right(60),
                DialRotations::Left(55),
                DialRotations::Left(1),
                DialRotations::Left(99),
                DialRotations::Right(14),
                DialRotations::Left(82),
            ]
        );
    }

    #[test]
    fn test_execute_rotations() {
        let positions = exectute_dial_rotation(
            50,
            &vec![
                DialRotations::Left(68),
                DialRotations::Left(30),
                DialRotations::Right(48),
                DialRotations::Left(5),
                DialRotations::Right(60),
                DialRotations::Left(55),
                DialRotations::Left(1),
                DialRotations::Left(99),
                DialRotations::Right(14),
                DialRotations::Left(82),
            ],
        );

        assert_eq!(
            positions,
            vec![
                (50, 0),
                (82, 1),
                (52, 0),
                (0, 1),
                (95, 0),
                (55, 1),
                (0, 1),
                (99, 0),
                (0, 1),
                (14, 0),
                (32, 1)
            ]
        );
    }

    #[test]
    fn test_password() {
        let dial_positions = vec![
            (50, 0),
            (82, 1),
            (52, 0),
            (0, 1),
            (95, 0),
            (55, 1),
            (0, 1),
            (99, 0),
            (0, 1),
            (14, 0),
            (32, 1),
        ];
        let password = generate_password(&dial_positions);
        assert_eq!(password, 3)
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_password_0x434C49434B() {
        let dial_positions = vec![
            (50, 0),
            (82, 1),
            (52, 0),
            (0, 1),
            (95, 0),
            (55, 1),
            (0, 1),
            (99, 0),
            (0, 1),
            (14, 0),
            (32, 1),
        ];
        let password = generate_password_0x434C49434B(&dial_positions);
        assert_eq!(password, 6)
    }
}
