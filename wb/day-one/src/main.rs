use std::collections::HashMap;
use std::fs;

fn main() {
    #[cfg(feature = "dev")]
    println!("Running in DEV mode");

    #[cfg(feature = "dev")]
    const FILE_NAME: &str = "src/test.txt";

    #[cfg(feature = "prod")]
    const FILE_NAME: &str = "src/input.txt";

    let (mut left, mut right) = read_vecs_from_file(FILE_NAME);
    left.sort();
    right.sort();

    calc_total_dist(&left, &right);
    calc_similarity_score(left, right);
}

fn calc_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let r_hash_map = group_duplicates(&right);
    let mut similarity_score: i32 = 0;
    for l in left {
        if let Some(duplicates) = r_hash_map.get(&l) {
            #[cfg(feature = "dev")]
            println!(
                "the key: {}, and the duplicate length: {}",
                l,
                (duplicates.len() as i32)
            );
            similarity_score += l * (duplicates.len() as i32);
        }
    }
    println!("Similarity score is {}", similarity_score);
    return similarity_score;
}

fn group_duplicates(sorted_list: &[i32]) -> HashMap<i32, Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for &value in sorted_list {
        map.entry(value).or_insert_with(Vec::new).push(value);
    }

    map
}

fn calc_total_dist(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let total_dist: i32 = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());
    println!("Total distance is: {}", total_dist);
    return total_dist;
}

fn read_vecs_from_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::read_to_string(path).expect("Could not read file");
    let lines: Vec<&str> = file.lines().collect();

    let mut v_left: Vec<i32> = Vec::new();
    let mut v_right: Vec<i32> = Vec::new();

    for line in lines {
        #[cfg(feature = "dev")]
        {
            println!("{}", line);
        }
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            v_left.push(left.parse::<i32>().expect("Invalid number"));
            v_right.push(right.parse::<i32>().expect("Invalid number"));
        }
    }
    #[cfg(feature = "dev")]
    {
        println!("This is the left vector: {:?}", v_left);
    }
    (v_left, v_right)
}
