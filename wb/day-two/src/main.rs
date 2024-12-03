use std::fs;

const MAX_DIFF: u32 = 3;
const MIN_DIFF: u32 = 1;

fn read_vecs_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = fs::read_to_string(path).expect("Could not read file");
    let lines: Vec<&str> = file.lines().collect();

    let mut vec_matrix: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        #[cfg(feature = "dev")]
        println!("{}", line);

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

fn has_safe_diffs(report: &Vec<i32>) -> bool {
    let abs_diffs = report.windows(2).map(|w| w[0].abs_diff(w[1]));
    let max_diff = abs_diffs.clone().max().unwrap();
    let min_diff = abs_diffs.min().unwrap();

    return (max_diff <= MAX_DIFF) & (min_diff >= MIN_DIFF);
}

fn is_monotonic(report: &Vec<i32>) -> bool {
    let decreasing = report.windows(2).all(|window| window[0] > window[1]);
    let increasing = report.windows(2).all(|window| window[0] < window[1]);
    decreasing | increasing
}

fn get_safety_vecs(inp_matrix: &Vec<Vec<i32>>) -> (Vec<bool>, Vec<bool>) {
    let mut safe_vec_fst_star = Vec::new();
    let mut safe_vec_snd_star = Vec::new();
    for x in inp_matrix {
        let is_safe = has_safe_diffs(&x) & is_monotonic(&x);
        safe_vec_fst_star.push(is_safe);
        if !is_safe {
            let dampened_reports_matrix = create_all_problem_dampened_reports(x);
            let safe_reports: Vec<_> = dampened_reports_matrix
                .iter()
                .filter(|dp| (has_safe_diffs(dp) & is_monotonic(dp)))
                .collect();
            safe_vec_snd_star.push(safe_reports.len() != 0);
        }else {
            safe_vec_snd_star.push(is_safe);
        }
    }
    (safe_vec_fst_star, safe_vec_snd_star)
}

fn create_all_problem_dampened_reports(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut dampened_reports: Vec<Vec<i32>> = Vec::new();
    for (i, _) in report.iter().enumerate() {
        let mut report_clone = report.clone();
        report_clone.remove(i);
        dampened_reports.push(report_clone);
    }
    #[cfg(feature = "dev")]
    {
        for dp in &dampened_reports {
            println!("Dampened reports: {:?}", dp);
        }
        println!("End of dampened reports")
    }
    return dampened_reports;
}

fn main() {
    #[cfg(feature = "dev")]
    const FILE_NAME: &str = "test.txt";

    #[cfg(feature = "prod")]
    const FILE_NAME: &str = "input.txt";

    let vec_matrix = read_vecs_from_file(FILE_NAME);
    let (safety_vec_fst_star, safety_vec_snd_star) = get_safety_vecs(&vec_matrix);
    let num_safe_reports_fst_star =
        safety_vec_fst_star
            .iter()
            .fold(0, |acc, x| if *x { acc + 1 } else { acc });

    let num_safe_reports_snd_star =
        safety_vec_snd_star
            .iter()
            .fold(0, |acc, x| if *x { acc + 1 } else { acc });

    println!(
        "There are originally {} safe reports",
        num_safe_reports_fst_star
    );
    println!(
        "After problem dampening, there are {} safe reports",
        num_safe_reports_snd_star
    );
}
