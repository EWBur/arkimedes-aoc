use std::collections::HashMap;
use std::fs;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

fn parse_numbers(line: &str, separator: char) -> Vec<u32> {
    line.split(separator)
        .map(|p| p.parse::<u32>().expect("Invalid number"))
        .collect()
}

fn read_rules_and_instructions(path: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let file = fs::read_to_string(path).expect("Could not read file");
    let mut order_rules = HashMap::new();
    let mut instructions = Vec::new();
    let mut rules_done = false;
    let separator = |done: bool| if done { ',' } else { '|' };

    for line in file.lines() {
        log::debug!("Processing line: {}", line);

        if line.is_empty() {
            log::info!("Detected empty line, switching to instruction parsing");
            rules_done = true;
            continue;
        }

        let numbers = parse_numbers(line, separator(rules_done));
        if !rules_done {
            order_rules
                .entry(numbers[1])
                .or_insert_with(Vec::new)
                .push(numbers[0]);
        } else {
            instructions.push(numbers);
        }
    }
    log::info!("Parsed {} rules and {} instructions", order_rules.len(), instructions.len());
    (order_rules, instructions)
}

fn validate_instruction(
    instruction: &[u32],
    rules: &HashMap<u32, Vec<u32>>,
) -> (bool, usize, usize) {
    log::debug!("Validating instruction: {:?}", instruction);
    let indices: HashMap<u32, usize> = instruction
        .iter()
        .enumerate()
        .map(|(i, &step)| (step, i))
        .collect();

    for (i, &step) in instruction.iter().enumerate() {
        if let Some(pre_steps) = rules.get(&step) {
            if let Some(violating_step) = pre_steps
                .iter()
                .find(|&&ps| indices.get(&ps).map_or(false, |&index| index > i))
            {
                log::debug!(
                    "Validation failed: step {} requires step {} to occur before index {}",
                    step,
                    violating_step,
                    i
                );
                return (false, i, *indices.get(violating_step).unwrap());
            }
        }
    }
    log::debug!("Validation successful for instruction: {:?}", instruction);
    (true, 0, 0)
}


fn correct_order(instruction: &[u32], rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut instr_clone = instruction.to_vec();
    while let (false, fst, snd) = validate_instruction(&instr_clone, rules) {
        log::debug!(
            "Before swap: {:?}, swapping indices {} and {}",
            instr_clone, fst, snd
        );

        instr_clone.swap(fst, snd);

        log::debug!("After swap: {:?}", instr_clone);
    }
    instr_clone
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let (rules, instructions) = read_rules_and_instructions(FILE_NAME);

    let validation_results: Vec<_> = instructions
        .iter()
        .map(|instr| validate_instruction(instr, &rules).0)
        .collect();

    let first_star_result: u32 = instructions
        .iter()
        .zip(validation_results.iter())
        .map(|(instr, &valid)| if valid { instr[instr.len() / 2] } else { 0 })
        .sum();

    log::info!("First star result: {}", first_star_result);

    let second_star_result: u32 = instructions
        .iter()
        .filter(|instr| !validate_instruction(instr, &rules).0)
        .map(|instr| correct_order(instr, &rules)[instr.len() / 2])
        .sum();

    log::info!("Second star result: {}", second_star_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_instruction() {
        let (rules, instructions) = read_rules_and_instructions("test.txt");
        let expected = vec![true, true, true, false, false, false];
        let mut output: Vec<bool> = Vec::new();
        for instr in instructions {
            output.push(validate_instruction(&instr, &rules).0);
        }
        assert_eq!(expected, output);
    }
}
