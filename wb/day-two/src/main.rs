use std::fs;

fn read_vecs_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = fs::read_to_string(path).expect("Could not read file");
    let lines: Vec<&str> = file.lines().collect();

    let mut vec_matrix: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        #[cfg(feature = "dev")]
        {
            println!("{}", line);
        }
        let mut parts = line.split_whitespace();
        let mut number_vec = Vec::new();
        while let Some(p) = parts.next() {
            number_vec.push(p.parse::<i32>().expect("Invalid Number"));
        }
        vec_matrix.push(number_vec);
    }
    #[cfg(feature = "dev")]
    println!("This is the vector matrix: {:?}", vec_matrix);

    vec_matrix
}

fn check_less_than_three(report: Vec<i32>) -> bool {
    let mut three_or_less = true;

    for (i, x) in report.iter().enumerate() {
        if i == report.len() - 1 {
            break;
        }
        #[cfg(feature = "dev")]
        println!("{}", (x - report[i + 1]).abs());
        let abs_diff = (x - report[i + 1]).abs();
        three_or_less = three_or_less & (abs_diff <= 3) & (abs_diff != 0);
    }
    return three_or_less;
}

fn is_monotonic(report: Vec<i32>) -> bool {
    let decreasing = report.windows(2).all(|window| window[0] > window[1]);
    let increasing = report.windows(2).all(|window| window[0] < window[1]);
    decreasing | increasing
}

fn get_safety_vec(inp_matrix: Vec<Vec<i32>>) -> Vec<bool> {
    let mut safe_vec = Vec::new();
    for x in inp_matrix {
        safe_vec.push(check_less_than_three(x.clone()) & is_monotonic(x));
    }
    safe_vec
}

fn main() {
    #[cfg(feature = "dev")]
    const FILE_NAME: &str = "test.txt";

    #[cfg(feature = "prod")]
    const FILE_NAME: &str = "input.txt";

    let vec_matrix = read_vecs_from_file(FILE_NAME);
    let safety_vec = get_safety_vec(vec_matrix);
    let mut num_safe_reports = 0;
    for s in safety_vec {
        if s {
            num_safe_reports += 1;
        }
    }

    println!("There are {} safe reports", num_safe_reports);
}
