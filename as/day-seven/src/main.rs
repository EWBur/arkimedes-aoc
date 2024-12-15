use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_lines<P>(input_file: P) -> std::io::Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_file).unwrap();
    BufReader::new(file).lines()
}

// Gives back a hasmap between a target value and operand strings
// Operands are kept as strings as interspersing operators becomes easier than if we first convert
// them to numeric values.
fn parse_input(lines: std::io::Lines<BufReader<File>>) -> Vec<(u64, Vec<String>)> {
    let mut input_lines: Vec<(u64, Vec<String>)> = Vec::new();

    for line in lines.flatten() {
        let parts: Vec<String> = line.split(':').map(|s| String::from(s)).collect();
        let target: u64 = parts[0].parse().unwrap();

        let operands: Vec<String> = parts[1]
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| String::from(s))
            .collect();

        input_lines.push((target, operands));
    }

    input_lines
}

fn find_operator_index(partial_expression: &Vec<String>) -> Option<usize> {
    let mut add_idx = None;
    let mut mul_idx = None;

    for (idx, elem) in partial_expression.iter().enumerate() {
        if elem == "+" {
            add_idx = Some(idx)
        }

        if elem == "*" {
            mul_idx = Some(idx)
        }
    }

    if let Some(add_idx) = add_idx {
        if mul_idx.is_none() {
            return Some(add_idx);
        }

        return Some(add_idx.max(mul_idx.unwrap()));
    }

    mul_idx
}

fn find_insertion_index(partial_expression: &Vec<String>) -> Option<usize> {
    let highest_operator_index = find_operator_index(partial_expression).unwrap_or(0);

    for (idx, elem) in partial_expression
        .iter()
        .enumerate()
        .skip(highest_operator_index)
    {
        if elem != "*" && elem != "+" {
            return Some(idx + 1);
        }
    }

    None
}

// Will also eventually generate the full expression
fn generate_partial_expressions(partial_expression: &Vec<String>) -> Vec<Vec<String>> {
    let mut resulting_expressions: Vec<Vec<String>> = Vec::new();

    match find_insertion_index(&partial_expression) {
        Some(idx) => {
            let mut with_mul: Vec<String> = partial_expression.clone();
            with_mul.insert(idx, String::from("*"));

            let mut with_add: Vec<String> = partial_expression.clone();
            with_add.insert(idx, String::from("+"));

            resulting_expressions.push(with_add);
            resulting_expressions.push(with_mul);
        }
        None => resulting_expressions.push(partial_expression.clone()),
    }

    resulting_expressions
}

fn evaluate_expression(expression: Vec<String>) -> u64 {
    let mut acc: u64 = expression[0].parse().unwrap();

    // Have to skip one element, as we use the first element as the accumulator
    for w in expression[1..].windows(2).step_by(2) {
        match w[0].as_str() {
            "*" => {
                let op: u64 = w[1].parse().unwrap();
                acc *= op;
            }
            "+" => {
                let op: u64 = w[1].parse().unwrap();
                acc += op;
            }
            _ => assert!(false, "This should never happen!")
        }

    }

    acc
}

fn is_result_possible(target_value: u64, operands: &Vec<String>) -> bool {
    let mut partial_expressions = generate_partial_expressions(operands);

    for _ in 0..operands.len() - 2 {
        let prutt: Vec<Vec<String>> = partial_expressions
            .iter()
            .flat_map(|expr| generate_partial_expressions(expr))
            .collect();

        partial_expressions = prutt;
    }

    for final_expression in partial_expressions {
        let evaluated_value = evaluate_expression(final_expression);
        if evaluated_value == target_value {
            return true;
        }
    }

    false
}

fn main() {
    let lines = read_lines("input.txt");
    let line_map = parse_input(lines);

    let mut total_value: u64 = 0;

    for (target_value, operands) in line_map.iter() {
        if is_result_possible(*target_value, operands) {
            total_value += target_value;
        }
    }

    // 12839601725877
    println!("Total value: {}", total_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        let mul_expression = vec![String::from("10"), String::from("*"), String::from("19")];
        assert_eq!(evaluate_expression(mul_expression), 190);

        let add_expression = vec![String::from("10"), String::from("+"), String::from("19")];
        assert_eq!(evaluate_expression(add_expression), 29);

        let add_expression = vec![
            String::from("1"),
            String::from("+"),
            String::from("2"),
            String::from("+"),
            String::from("3"),
        ];
        assert_eq!(evaluate_expression(add_expression), 6);
    }

    #[test]
    fn find_highest_operator_index() {
        let mul_expression = vec![String::from("10"), String::from("*"), String::from("19")];
        assert_eq!(find_operator_index(&mul_expression), Some(1));
        let mul_expression = vec![
            String::from("10"),
            String::from("*"),
            String::from("19"),
            String::from("+"),
            String::from("19"),
        ];
        assert_eq!(find_operator_index(&mul_expression), Some(3));
        let mul_expression = vec![
            String::from("10"),
            String::from("*"),
            String::from("19"),
            String::from("+"),
            String::from("19"),
            String::from("*"),
            String::from("19"),
        ];
        assert_eq!(find_operator_index(&mul_expression), Some(5));
    }

    #[test]
    fn test_find_insertion_index() {
        let mul_expression = vec![String::from("10"), String::from("19")];
        assert_eq!(find_insertion_index(&mul_expression), Some(1));
        let mul_expression = vec![String::from("10"), String::from("+"), String::from("19")];
        assert_eq!(find_insertion_index(&mul_expression), Some(3));
    }
}
