use regex::Regex;
use std::fs;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";

#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

fn read_string_from_file(path: &str) -> String {
    let input_string = fs::read_to_string(path).expect("Could not read file");

    input_string.to_string()
}

fn remove_unnecessary_chars(multiplications: &Vec<&str>, bad_chars: &Vec<char>) -> Vec<String> {
    multiplications
        .iter()
        .map(|x| x.replace(&bad_chars[..], ""))
        .collect()
}

fn get_sum_of_mults(mults: &Vec<String>) -> i32 {
    mults.iter().fold(0, |acc, x| {
        let nums: Vec<i32> = x.split(',').map(|s| s.parse().unwrap()).collect();
        acc + nums.iter().product::<i32>()
    })
}

fn main() {
    let input_string = read_string_from_file(FILE_NAME);

    #[cfg(feature = "dev")]
    println!("The input string: {}", input_string);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let found_multiplications: Vec<&str> =
        re.find_iter(&input_string).map(|m| m.as_str()).collect();

    #[cfg(feature = "dev")]
    println!("Found multiplications: {:?}", found_multiplications);

    let bad_chars = vec!['m', 'u', 'l', '(', ')'];
    let cleaned_mults: Vec<String> = remove_unnecessary_chars(&found_multiplications, &bad_chars);

    #[cfg(feature = "dev")]
    println!("Cleaned mults: {:?}", &cleaned_mults);

    let result = get_sum_of_mults(&cleaned_mults);

    println!("The sum of all multiplications is: {}", result);

}
