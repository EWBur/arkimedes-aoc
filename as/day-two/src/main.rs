use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const LARGEST_DIFF: u32 = 3;
const SMALLEST_DIFF: u32 = 1;

fn get_largest_difference(values: &Vec<u32>) -> u32 {
    values
        .windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .max()
        .unwrap()
}

fn get_smallest_difference(values: &Vec<u32>) -> u32 {
    values
        .windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .min()
        .unwrap()
}

fn vals_are_all_increasing(values: &Vec<u32>) -> bool {
    values.windows(2).all(|window| window[0] < window[1])
}

fn vals_are_all_decreasing(values: &Vec<u32>) -> bool {
    values.windows(2).all(|window| window[0] > window[1])
}

fn is_safe(values: &Vec<u32>) -> bool {
    (vals_are_all_decreasing(&values) || vals_are_all_increasing(&values))
        && get_largest_difference(&values) <= LARGEST_DIFF
        && get_smallest_difference(&values) >= SMALLEST_DIFF
}

fn is_safe_with_one_invalid_dropped(values: &Vec<u32>) -> bool {
    generate_subvectors(values)
        .iter()
        .filter(|values| is_safe(&values))
        .count()
        >= 1
}

fn generate_subvectors(values: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut v = Vec::new();
    for (i, _) in values.iter().enumerate() {
        let mut v_clone = values.clone();
        _ = v_clone.remove(i);
        v.push(v_clone);
    }

    v
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").unwrap();

    let parsed_lines: Vec<Vec<u32>> = lines
        .flatten()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_count = parsed_lines.iter().filter(|vals| is_safe(vals)).count();

    // 411
    println!("Safe count: {}", safe_count);

    let bad_lines: Vec<Vec<u32>> = parsed_lines
        .iter()
        .filter(|values| !is_safe(values))
        .map(|values| values.clone())
        .collect();

    let safe_with_one_removed = bad_lines
        .iter()
        .filter(|vals| is_safe_with_one_invalid_dropped(&vals))
        .count();

    // 465
    println!(
        "Additional safes: {}, total: {}",
        safe_with_one_removed,
        safe_with_one_removed + safe_count,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_largest_difference() {
        let vals = vec![1, 2, 3];
        assert_eq!(get_largest_difference(&vals), 1);

        let vals = vec![1, 10, 12];
        assert_eq!(get_largest_difference(&vals), 9);
    }

    #[test]
    fn test_get_smallest_difference() {
        let vals = vec![1, 2, 3];
        assert_eq!(get_smallest_difference(&vals), 1);

        let vals = vec![1, 10, 12];
        assert_eq!(get_smallest_difference(&vals), 2);
    }
}
