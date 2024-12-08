#![allow(unused)]

use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// (0,0) is top-left corner
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Map {
    tiles: Vec<Vec<char>>,
    initial_position: Position,
    initial_direction: Direction,
}

impl Map {
    fn from_file<P>(input_file: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let mut initial_position = None;
        let mut parsed_tiles: Vec<Vec<char>> = Vec::new();

        let file = File::open(input_file).expect("Failed to open input file!");

        let lines = BufReader::new(file).lines();

        for (row_num, contents) in lines.flatten().enumerate() {
            if let Some(col_num) = contents.find('^') {
                initial_position = Some(Position::new(row_num, col_num));
            }
            parsed_tiles.push(contents.chars().collect());
        }

        assert!(initial_position != None);

        Self {
            tiles: parsed_tiles,
            initial_position: initial_position.unwrap(),
            initial_direction: Direction::Up,
        }
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        let row: &Vec<char> = self.tiles.get(row).unwrap();
        *row.get(col).unwrap()
    }
}

fn move_up_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashSet<Position>,
) -> Option<(Position, Direction)> {
    let mut current_row = starting_position.row;

    while current_row > 0 {
        let next_row = current_row - 1;
        if map.char_at(next_row, starting_position.col) == '#' {
            return Some((
                Position::new(current_row, starting_position.col),
                direction.turn_right(),
            ));
        }

        positions_visited.insert(Position::new(current_row, starting_position.col));
        current_row = next_row;
    }

    positions_visited.insert(Position::new(current_row, starting_position.col));
    None
}

fn move_down_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashSet<Position>,
) -> Option<(Position, Direction)> {
    let mut current_row = starting_position.row;
    let bottom_row_num = map.tiles.len() - 1;

    while current_row < bottom_row_num {
        let next_row = current_row + 1;
        if map.char_at(next_row, starting_position.col) == '#' {
            return Some((
                Position::new(current_row, starting_position.col),
                direction.turn_right(),
            ));
        }

        positions_visited.insert(Position::new(current_row, starting_position.col));
        current_row = next_row;
    }

    positions_visited.insert(Position::new(current_row, starting_position.col));
    None
}

fn move_right_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashSet<Position>,
) -> Option<(Position, Direction)> {
    let mut current_col = starting_position.col;
    let rightmost_col_num = map.tiles.get(0).unwrap().len() - 1;

    while current_col < rightmost_col_num {
        let next_col = current_col + 1;
        if map.char_at(starting_position.row, next_col) == '#' {
            return Some((
                Position::new(starting_position.row, current_col),
                direction.turn_right(),
            ));
        }

        positions_visited.insert(Position::new(starting_position.row, current_col));
        current_col = next_col;
    }

    positions_visited.insert(Position::new(starting_position.row, current_col));
    None
}

fn move_left_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashSet<Position>,
) -> Option<(Position, Direction)> {
    let mut current_col = starting_position.col;

    while current_col > 0 {
        let next_col = current_col - 1;
        if map.char_at(starting_position.row, next_col) == '#' {
            return Some((
                Position::new(starting_position.row, current_col),
                direction.turn_right(),
            ));
        }

        positions_visited.insert(Position::new(starting_position.row, current_col));
        current_col = next_col;
    }

    positions_visited.insert(Position::new(starting_position.row, current_col));

    None
}

fn move_from(
    starting_position: Position,
    initial_direction: Direction,
    map: &Map,
    positions_visited: &mut HashSet<Position>,
) -> Option<(Position, Direction)> {
    match initial_direction {
        Direction::Up => move_up_from(starting_position, initial_direction, map, positions_visited),
        Direction::Right => {
            move_right_from(starting_position, initial_direction, map, positions_visited)
        }
        Direction::Down => {
            move_down_from(starting_position, initial_direction, map, positions_visited)
        }
        Direction::Left => {
            move_left_from(starting_position, initial_direction, map, positions_visited)
        }
    }
}

fn main() {
    let map = Map::from_file("input.txt");
    let mut position = map.initial_position;
    let mut direction = map.initial_direction.clone();

    let mut positions_visited = HashSet::new();

    loop {
        match move_from(position, direction, &map, &mut positions_visited) {
            Some((new_position, new_direction)) => {
                position = new_position;
                direction = new_direction;
            }
            None => break,
        }
    }

    println!("{}", positions_visited.len());
}
