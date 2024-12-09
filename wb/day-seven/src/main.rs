use std::fs;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

fn parse_lines(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let res: u64 = parts[0].parse().expect("Failed to parse res");
    let parts: Vec<u64> = parts[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("Failed to parse part"))
        .collect();

    (res,parts)
}

fn read_calibration_eqs(file_path: &str) -> (Vec<u64>,Vec<Vec<u64>>) {
    let file = fs::read_to_string(file_path).expect("Could not read file.");
    let mut equations: Vec<Vec<u64>> = Vec::new();
    let mut results: Vec<u64> = Vec::new();

    for line in file.lines() {
        let (res,parts) = parse_lines(line);
        results.push(res);
        equations.push(parts)
    }
    (results, equations)
}

fn generate_combinations(input: Vec<&str>, length: usize) -> Vec<Vec<&str>> {
    if length == 0 {
        return vec![vec![]];
    }
    let mut result = vec![];
    for &item in &input {
        let sub_combinations = generate_combinations(input.clone(), length - 1);
        for mut combination in sub_combinations {
            combination.insert(0, item);
            result.push(combination);
        }
    }
    result
}

fn apply_operators(ops: &Vec<&str>, eq: &Vec<u64>) -> u64 {
    let mut res = eq[0];
    for (i,op) in ops.iter().enumerate() {
        res = match *op {
            "add" => res + eq[i + 1],
            "mul" => res * eq[i + 1],
            "concat" => (res.to_string() + &eq[i+1].to_string()).parse::<u64>().expect("bad conversion"),
            _ => panic!("Unknown operator"),
        };
    }
    res
}

fn check_correct_combinations(res: &u64, eq: &Vec<u64>) -> u64 {
    let combinations = generate_combinations(vec!["add","mul", "concat"], eq.len() - 1);
    let results: Vec<u64> =  combinations
            .iter()
            .map(|ops| apply_operators(ops, &eq)).collect();

    let checks_out = results
        .iter()
        .any(| x| *x == *res);
    if checks_out {return *res} else {return 0}
}

fn main() {
    let (results, equations) = read_calibration_eqs(FILE_NAME);

    let mut correct_combinations = 0;
    for (r,eq) in results.iter().zip(equations.iter()) {
        correct_combinations += check_correct_combinations(r, eq)
    }

    println!("The number of correct combinations are {}", correct_combinations);
}
