#![allow(unused)]

use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_file_as_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(path).unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // regex for mul(XYZ,XYZ) where XYZ is 1-3 digits, with two capture groups
    // for the digits, so we can extract them directly
    //let regex_mul_op = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    //let mut vals: Vec<u32> = Vec::new();
    //let lines = read_lines("input.txt").unwrap();
    //for line in lines.flatten() {
    //    for (_, [fst, snd]) in regex_mul_op
    //        .captures_iter(line.as_str())
    //        .map(|h| h.extract())
    //    {
    //        vals.push(fst.parse::<u32>().unwrap() * snd.parse::<u32>().unwrap());
    //    }
    //}

    //let total: u32 = vals.iter().sum();
    //println!("Total was: {}", total); // 188192787

    // for second star
    // Match either multiplication operations or do/dont, to disable/enable following
    // multiplications
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut vals = Vec::new();

    // Probably easier to just read the whole file as a single string than as lines
    let input_string = read_file_as_string("input.txt");
    let mut perform_multiplications = true;

    for needle in regex.captures_iter(&input_string) {
        if &needle[0] == "do()" || &needle[0] == "don't()" {
            perform_multiplications = &needle[0] == "do()";
        } else if perform_multiplications {
            vals.push(needle[1].parse::<u32>().unwrap() * needle[2].parse::<u32>().unwrap())
        }
    }
    let total: u32 = vals.iter().sum();
    println!("Total is: {}", total);
}
