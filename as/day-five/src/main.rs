use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn read_lines<P>(path: P) -> io::Result<std::io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn parse_input(lines: Lines<BufReader<File>>) -> (Vec<String>, Vec<String>) {
    let mut first_part = Vec::new();
    let mut second_part = Vec::new();

    let mut reading_first_part = true;

    for line in lines.map_while(Result::ok) {
        if !line.contains("|") && reading_first_part {
            reading_first_part = false;
            continue;
        }

        if reading_first_part {
            first_part.push(line);
        } else {
            second_part.push(line);
        }
    }

    (first_part, second_part)
}

fn build_precedence_map(number_orderings: Vec<String>) -> HashMap<u8, Vec<u8>> {
    let mut precedence_map: HashMap<u8, Vec<u8>> = HashMap::new();

    for line in number_orderings {
        let numbers: Vec<u8> = line.split("|").map(|n| n.parse::<u8>().unwrap()).collect();
        assert_eq!(numbers.len(), 2);

        precedence_map
            .entry(numbers[1])
            .and_modify(|v: &mut Vec<u8>| v.push(numbers[0]))
            .or_insert(Vec::from([numbers[0]]));
    }

    precedence_map
}

fn find_middle_number(numbers: Vec<u8>) -> u8 {
    let middle_idx = numbers.len() / 2;

    *numbers.get(middle_idx).unwrap()
}

fn are_valid_page_numbers(page_numbers: &[u8], precedence_map: &HashMap<u8, Vec<u8>>) -> bool {
    for (idx, number) in page_numbers.iter().enumerate() {
        match precedence_map.get(number) {
            Some(numbers_with_precedence) => {
                for n in &page_numbers[idx..] {
                    if numbers_with_precedence.contains(n) {
                        return false;
                    }
                }
            }
            None => continue,
        }
    }

    true
}

fn find_valid_page_numbers(
    page_numbers: Vec<Vec<u8>>,
    precedence_map: HashMap<u8, Vec<u8>>,
) -> Vec<Vec<u8>> {
    page_numbers
        .iter()
        .filter(|numbers| are_valid_page_numbers(numbers, &precedence_map))
        .map(|v| v.to_vec())
        .collect()
}

fn parse_page_numbers(page_numbers_input: Vec<String>) -> Vec<Vec<u8>> {
    let mut parsed_page_numbers: Vec<Vec<u8>> = Vec::new();

    for page_numbers in page_numbers_input {
        let page_numbers: Vec<u8> = page_numbers
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        parsed_page_numbers.push(page_numbers);
    }

    parsed_page_numbers
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let (first_part, second_part) = parse_input(lines);

    let page_numbers = parse_page_numbers(second_part);
    let precedence_map = build_precedence_map(first_part);

    let valid_page_numbers = find_valid_page_numbers(page_numbers, precedence_map);

    let total: u32 = valid_page_numbers
        .iter()
        .map(|nums| find_middle_number(nums.to_vec()) as u32)
        .sum();

    // 7074
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_valid_page_numbers() {
        let mut page_numbers = Vec::new();
        page_numbers.push(vec![1, 2, 3]);
        page_numbers.push(vec![1, 2, 3, 4]);
        page_numbers.push(vec![1, 3, 2]);

        let mut precedence_map = HashMap::new();
        precedence_map.insert(2, vec![1]);
        precedence_map.insert(3, vec![1, 2]);
        precedence_map.insert(4, vec![1, 2, 3]);

        let valids = find_valid_page_numbers(page_numbers, precedence_map);

        assert_eq!(valids.len(), 2);
    }
}
