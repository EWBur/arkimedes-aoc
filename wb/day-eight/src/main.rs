use itertools::iproduct;
use std::{collections::HashMap,collections::HashSet, fs};

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn generate_antinode(&self, other: &Point) -> Point {
        Point {
            x: 2 * self.x - other.x,
            y: 2 * self.y - other.y,
        }
    }
}

fn read_matrix(path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(path).expect("Could not read file");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        matrix.push(line.chars().collect());
    }
    matrix
}

fn find_antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != '.' {
                antennas
                    .entry(*col)
                    .or_insert_with(Vec::new)
                    .push(Point::new(j as isize, i as isize));
            }
        }
    }
    antennas
}

fn in_bounds(position: Point, x_lim: isize, y_lim: isize) -> bool {
    let x_in_bound = (position.x >= 0) && (position.x < x_lim);
    let y_in_bound = (position.y >= 0) && (position.y < y_lim);
    x_in_bound && y_in_bound
}

fn get_all_antinodes(map: &Vec<Vec<char>>, antennas: &HashMap<char, Vec<Point>>) -> Vec<Point> {
    let antenna_values: Vec<&Vec<Point>> = antennas.values().collect();
    let x_lim = map.len();
    let y_lim = map[0].len();
    let mut antinodes: Vec<Point> = Vec::new();
    for antenna_types in antenna_values {
        for (a1, a2) in iproduct!(
            antenna_types,
            antenna_types
        ) {
            if a1 != a2 {
                let an = a1.generate_antinode(a2);
                if in_bounds(an, x_lim as isize, y_lim as isize)
                {
                    log::debug!("Found antinode!\n AN x: {}, y: {}\n fst x: {}, y: {}\n snd x: {}, y: {}\n values {}, {}", an.x,an.y,a1.x,a1.y,a2.x,a2.y, map[a1.y as usize][a1.x as usize], map[a2.y as usize][a2.x as usize]);
                    antinodes.push(an);
                }
            }
        }
    }
    
    antinodes
}

fn visualize_antinodes(mut map: Vec<Vec<char>>, antennas: &Vec<Point>) {
    for a in antennas {
        map[a.y as usize][a.x as usize] = '#';
    }
    for row in map {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let antenna_map: Vec<Vec<char>> = read_matrix(FILE_NAME);
    let antennas: HashMap<char, Vec<Point>> = find_antennas(&antenna_map);
    let antinodes = get_all_antinodes(&antenna_map, &antennas);

    visualize_antinodes(antenna_map.clone(), &antinodes);

    let unique_antinodes: HashSet<_> = antinodes.into_iter().collect();
    println!("The number of antinodes is {}", unique_antinodes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_matrix() {
        let path = "test.txt";
        let matrix = read_matrix(path);
        assert_eq!(matrix.len(), 12);
        assert_eq!(matrix[0].len(), 12);
    }

    #[test]
    fn test_find_antennas() {
        let map = vec![
            vec!['.', 'A', '.'],
            vec!['B', '.', 'C'],
            vec!['.', 'D', '.'],
        ];
        let antennas = find_antennas(&map);
        assert_eq!(antennas.len(), 4);
        assert_eq!(antennas[&'A'], vec![Point::new(1, 0)]);
        assert_eq!(antennas[&'B'], vec![Point::new(0, 1)]);
        assert_eq!(antennas[&'C'], vec![Point::new(2, 1)]);
        assert_eq!(antennas[&'D'], vec![Point::new(1, 2)]);
    }
}
