use std::fs;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

fn read_string_matrix(file_path: &str) -> Vec<Vec<char>> {
    let string_input = fs::read_to_string(file_path).expect("Failed to read file");
    return string_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}

fn generate_direction_vecs(input: &[i32]) -> Vec<[i32; 2]> {
    let mut combinations = vec![];
    for &i in input {
        for &j in input {
            if !(i == 0 && j == 0) {
                combinations.push([i, j]);
            }
        }
    }
    combinations
}

fn check_in_bound(i: usize, j: usize, m: &Vec<Vec<char>>, dv: &[i32; 2]) -> bool {
    let new_row = i as i32 + dv[0] * 3;
    let new_col = j as i32 + dv[1] * 3;

    let row_positive_dir = dv[0].is_positive();
    let row_negative_within_bounds = (dv[0] * 3).abs() <= i as i32;
    let row_in_bound =
        (new_row < m.len() as i32) && (row_positive_dir || row_negative_within_bounds);

    let col_positive_dir = dv[1].is_positive();
    let col_negative_within_bounds = (dv[1] * 3).abs() <= j as i32;
    let col_in_bound =
        (new_col < m[i].len() as i32) && (col_positive_dir || col_negative_within_bounds);

    row_in_bound && col_in_bound
}

fn star_check(
    i: usize,
    j: usize,
    m: &Vec<Vec<char>>,
    check: [char; 3],
    dir_vecs: &Vec<[i32; 2]>,
) -> u32 {
    let mut found_xmas = 0;
    let muls = [1, 2, 3];

    'dv_loop: for dv in dir_vecs.iter() {
        if check_in_bound(i, j, m, dv) {
            for (factor, xc) in muls.iter().zip(check.iter()) {
                let row_i = (i as i32 + dv[0] * factor) as usize;
                let col_i = (j as i32 + dv[1] * factor) as usize;
                if m[row_i][col_i] == *xc {
                    if *factor == 3 {
                        found_xmas += 1;
                        #[cfg(feature = "dev")]
                        println!("Found at {},{}", i, j);
                    }
                } else {
                    continue 'dv_loop;
                }
            }
        }
    }
    found_xmas
}

fn count_xmas(matrix: &Vec<Vec<char>>) -> u32 {
    let mut n_xmas: u32 = 0;
    let xmas_check = ['M', 'A', 'S'];
    let direction_vecs = generate_direction_vecs(&vec![1, 0, -1]);
    for (i, _) in matrix.iter().enumerate() {
        for (j, _) in matrix[i].iter().enumerate() {
            if matrix[i][j] == 'X' {
                n_xmas += star_check(i, j, matrix, xmas_check, &direction_vecs);
            }
        }
    }
    n_xmas
}

fn generate_rotations<T: Clone>(input: &[T]) -> Vec<Vec<T>> {
    let mut rotations = Vec::new();
    let len = input.len();
    for i in 0..len {
        let rotation = input[i..]
            .iter()
            .chain(input[..i].iter())
            .cloned()
            .collect();
        rotations.push(rotation);
    }
    rotations
}

fn get_corners(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    vec![
        matrix[i - 1][j + 1],
        matrix[i + 1][j + 1],
        matrix[i + 1][j - 1],
        matrix[i - 1][j - 1],
    ]
}

fn count_xs_of_mas(matrix: &Vec<Vec<char>>) -> u32 {
    let mut n_xs_of_mas: u32 = 0;
    let corner_patterns = generate_rotations(&vec!['M', 'M', 'S', 'S']);
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] == 'A' {
                let corners = get_corners(i, j, matrix);
                if corner_patterns.iter().any(|cp| &corners == cp) {
                    n_xs_of_mas += 1;
                    #[cfg(feature = "dev")]
                    println!("Found one at i: {}, j: {}", i, j)
                }
            }
        }
    }
    n_xs_of_mas
}

fn main() {
    let xmas_matrix = read_string_matrix(FILE_NAME);
    let xmas_result = count_xmas(&xmas_matrix);
    let xs_of_mas_result = count_xs_of_mas(&xmas_matrix);
    println!("xmasses: {}, xs of mas: {}", xmas_result, xs_of_mas_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_direction_vecs() {
        let input = vec![1, 0, -1];
        let expected = vec![
            [1, 1],
            [1, 0],
            [1, -1],
            [0, 1],
            [0, -1],
            [-1, 1],
            [-1, 0],
            [-1, -1],
        ];
        assert_eq!(generate_direction_vecs(&input), expected);
    }

    #[test]
    fn test_check_in_bound() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['M', 'A', 'S', 'X'],
            vec!['A', 'S', 'X', 'M'],
            vec!['S', 'X', 'M', 'A'],
        ];
        assert!(check_in_bound(0, 0, &matrix, &[1, 1]));
        assert!(!check_in_bound(0, 0, &matrix, &[3, 3]));
    }

    #[test]
    fn test_generate_rotations() {
        let input = vec!['M', 'M', 'S', 'S'];
        let expected = vec![
            vec!['M', 'M', 'S', 'S'],
            vec!['M', 'S', 'S', 'M'],
            vec!['S', 'S', 'M', 'M'],
            vec!['S', 'M', 'M', 'S'],
        ];
        assert_eq!(generate_rotations(&input), expected);
    }
}
