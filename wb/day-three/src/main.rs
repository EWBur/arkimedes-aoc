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

fn main() {
    let input_string = read_string_from_file(FILE_NAME);

    #[cfg(feature = "dev")]
    println!("The input string: {}", input_string);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut result = 0;
    let mut mul_activated = true;
    
    for caps in re.captures_iter(&input_string) {
        #[cfg(feature = "dev")]
        println!("{:?}", caps);
        
        if (&caps[0] == "do()") | (&caps[0] == "don't()"){
            mul_activated = &caps[0] == "do()";
        }else if mul_activated {
            let x = &caps[1]; 
            let y = &caps[2];
            result += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }
    println!("Result result: {}", result);
}
