use std::{iter::Rev, str::Chars};

#[derive(Debug, PartialEq)]
enum MathProblem {
    Add(Vec<u64>),
    Multiply(Vec<u64>),
}

impl MathProblem {
    fn solve(&self) -> u64 {
        match self {
            MathProblem::Add(numbers) => numbers.iter().sum(),
            MathProblem::Multiply(numbers) => numbers.iter().product(),
        }
    }
}

fn get_column_widths(operator_line: &str) -> Vec<usize> {
    let mut column_widths = Vec::new();

    let mut counter = 0;
    for c in operator_line.chars().skip(1) {
        if c.is_whitespace() {
            counter += 1;
        } else {
            column_widths.push(counter);
            counter = 0;
        }
    }

    column_widths.push(counter + 1);

    column_widths
}

fn parse_input(
    input: &str,
    column_parser: fn(column: Vec<String>, column_width: usize) -> Vec<u64>,
) -> Vec<MathProblem> {
    let mut math_problems = Vec::new();

    let lines = input.lines().collect::<Vec<&str>>();
    let mut number_lines = lines[0..lines.len() - 1]
        .iter()
        .map(|line| String::from(*line))
        .collect::<Vec<String>>();
    let mut operator_line = String::from(lines[lines.len() - 1]);

    for column_widh in get_column_widths(&operator_line) {
        let mut column_numbers = Vec::new();
        for number_line in number_lines.iter_mut() {
            let number = number_line.drain(0..column_widh).collect::<String>();
            column_numbers.push(number);

            if !number_line.is_empty() {
                number_line.remove(0);
            }
        }

        if operator_line.remove(0) == '+' {
            math_problems.push(MathProblem::Add(column_parser(column_numbers, column_widh)));
        } else {
            math_problems.push(MathProblem::Multiply(column_parser(
                column_numbers,
                column_widh,
            )));
        }

        if operator_line.len() > column_widh {
            operator_line.drain(0..column_widh);
        }
    }

    math_problems
}

fn parse_human_math(column: Vec<String>, _: usize) -> Vec<u64> {
    column
        .iter()
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect()
}

fn parse_cephalopod_math(column: Vec<String>, column_width: usize) -> Vec<u64> {
    let mut number_strings = Vec::new();
    let mut cells = column
        .iter()
        .map(|cell| cell.chars().rev())
        .collect::<Vec<Rev<Chars>>>();

    for _ in 0..column_width {
        number_strings.push(cells.iter_mut().flat_map(|cell| cell.next()).collect());
    }

    parse_human_math(number_strings, column_width)
}

fn main() {
    let input = include_str!("../inputs/data_day_6.txt");

    // Solution for puzzle 1
    let human_math_problems = parse_input(input, parse_human_math);
    let grand_total = human_math_problems
        .iter()
        .map(MathProblem::solve)
        .sum::<u64>();
    println!("The grand total of the worksheet with human math is {grand_total}");

    // Solution for puzzle 2
    let human_math_problems = parse_input(input, parse_cephalopod_math);
    let grand_total = human_math_problems
        .iter()
        .map(MathProblem::solve)
        .sum::<u64>();
    println!("The grand total of the worksheet with cephalopod math is {grand_total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_width() {
        let operator_line = "+   *    +    *  * ";
        assert_eq!(get_column_widths(operator_line), vec![3, 4, 4, 2, 2]);
    }

    #[test]
    fn test_parse_input_human_math() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        assert_eq!(
            parse_input(input, parse_human_math),
            vec![
                MathProblem::Multiply(vec![123, 45, 6]),
                MathProblem::Add(vec![328, 64, 98]),
                MathProblem::Multiply(vec![51, 387, 215]),
                MathProblem::Add(vec![64, 23, 314])
            ]
        );
    }

    #[test]
    fn test_parse_input_cephalopod_math() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        assert_eq!(
            parse_input(input, parse_cephalopod_math),
            vec![
                MathProblem::Multiply(vec![356, 24, 1]),
                MathProblem::Add(vec![8, 248, 369]),
                MathProblem::Multiply(vec![175, 581, 32]),
                MathProblem::Add(vec![4, 431, 623])
            ]
        );
    }

    #[test]
    fn test_solve_math_problem() {
        assert_eq!(MathProblem::Multiply(vec![123, 45, 6]).solve(), 33210);
        assert_eq!(MathProblem::Add(vec![328, 64, 98]).solve(), 490);
        assert_eq!(MathProblem::Multiply(vec![51, 387, 215]).solve(), 4243455);
        assert_eq!(MathProblem::Add(vec![64, 23, 314]).solve(), 401);

        assert_eq!(MathProblem::Multiply(vec![356, 24, 1]).solve(), 8544);
        assert_eq!(MathProblem::Add(vec![8, 248, 369]).solve(), 625);
        assert_eq!(MathProblem::Multiply(vec![175, 581, 32]).solve(), 3253600);
        assert_eq!(MathProblem::Add(vec![4, 431, 623]).solve(), 1058);
    }
}
