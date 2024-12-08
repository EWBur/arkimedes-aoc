#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

enum MovementResult {
    NewPosition {
        new_position: Position,
        new_direction: Direction,
    },
    OffTheMap,
    LoopFound,
}

#[derive(Clone)]
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

    fn set_char_to(&mut self, row: usize, col: usize, set_to: char) {
        self.tiles[row][col] = set_to;
    }
}

fn move_up_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> MovementResult {
    let mut current_row = starting_position.row;

    while current_row > 0 {
        let next_row = current_row - 1;
        if map.char_at(next_row, starting_position.col) == '#' {
            return MovementResult::NewPosition {
                new_position: Position::new(current_row, starting_position.col),
                new_direction: direction.turn_right(),
            };
        }

        let new_position = Position::new(current_row, starting_position.col);

        if found_loop(&new_position, &direction, positions_visited) {
            return MovementResult::LoopFound;
        }

        positions_visited
            .entry(new_position)
            .and_modify(|dirs| dirs.push(direction))
            .or_insert(vec![direction]);
        current_row = next_row;
    }

    positions_visited
        .entry(Position::new(current_row, starting_position.col))
        .and_modify(|dirs| dirs.push(direction))
        .or_insert(vec![direction]);
    MovementResult::OffTheMap
}

fn move_down_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> MovementResult {
    let mut current_row = starting_position.row;
    let bottom_row_num = map.tiles.len() - 1;

    while current_row < bottom_row_num {
        let next_row = current_row + 1;
        if map.char_at(next_row, starting_position.col) == '#' {
            return MovementResult::NewPosition {
                new_position: Position::new(current_row, starting_position.col),
                new_direction: direction.turn_right(),
            };
        }

        let new_position = Position::new(current_row, starting_position.col);

        if found_loop(&new_position, &direction, positions_visited) {
            return MovementResult::LoopFound;
        }

        positions_visited
            .entry(new_position)
            .and_modify(|dirs| dirs.push(direction))
            .or_insert(vec![direction]);
        current_row = next_row;
    }

    positions_visited
        .entry(Position::new(current_row, starting_position.col))
        .and_modify(|dirs| dirs.push(direction))
        .or_insert(vec![direction]);
    MovementResult::OffTheMap
}

fn move_right_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> MovementResult {
    let mut current_col = starting_position.col;
    let rightmost_col_num = map.tiles.get(0).unwrap().len() - 1;

    while current_col < rightmost_col_num {
        let next_col = current_col + 1;
        if map.char_at(starting_position.row, next_col) == '#' {
            return MovementResult::NewPosition {
                new_position: Position::new(starting_position.row, current_col),
                new_direction: direction.turn_right(),
            };
        }

        let new_position = Position::new(starting_position.row, current_col);

        if found_loop(&new_position, &direction, positions_visited) {
            return MovementResult::LoopFound;
        }

        positions_visited
            .entry(new_position)
            .and_modify(|dirs| dirs.push(direction))
            .or_insert(vec![direction]);
        current_col = next_col;
    }

    positions_visited
        .entry(Position::new(starting_position.row, current_col))
        .and_modify(|dirs| dirs.push(direction))
        .or_insert(vec![direction]);

    MovementResult::OffTheMap
}

fn move_left_from(
    starting_position: Position,
    direction: Direction,
    map: &Map,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> MovementResult {
    let mut current_col = starting_position.col;

    while current_col > 0 {
        let next_col = current_col - 1;
        if map.char_at(starting_position.row, next_col) == '#' {
            return MovementResult::NewPosition {
                new_position: Position::new(starting_position.row, current_col),
                new_direction: direction.turn_right(),
            };
        }
        let new_position = Position::new(starting_position.row, current_col);

        if found_loop(&new_position, &direction, positions_visited) {
            return MovementResult::LoopFound;
        }

        positions_visited
            .entry(new_position)
            .and_modify(|dirs| dirs.push(direction))
            .or_insert(vec![direction]);
        current_col = next_col;
    }

    positions_visited
        .entry(Position::new(starting_position.row, current_col))
        .and_modify(|dirs| dirs.push(direction))
        .or_insert(vec![direction]);

    MovementResult::OffTheMap
}

fn found_loop(
    position: &Position,
    direction: &Direction,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> bool {
    if let Some(directions) = positions_visited.get(position) {
        return directions.contains(direction);
    }

    false
}

fn move_from(
    starting_position: Position,
    initial_direction: Direction,
    map: &Map,
    positions_visited: &mut HashMap<Position, Vec<Direction>>,
) -> MovementResult {
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

fn generate_possible_obstacle_placements(original_map: &Map) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();

    for (row_num, row_contents) in original_map.tiles.iter().enumerate() {
        for (col_num, col_contents) in row_contents.iter().enumerate() {
            if *col_contents != '^' && *col_contents != '#' {
                let mut modified_map = original_map.clone();
                modified_map.set_char_to(row_num, col_num, '#');
                maps.push(modified_map);
            }
        }
    }

    maps
}

fn main() {
    let map = Map::from_file("input.txt");
    let mut position = map.initial_position;
    let mut direction = map.initial_direction.clone();

    // Map from a position to the directions in which the guard has moved when visiting the position
    let mut positions_visited: HashMap<Position, Vec<Direction>> = HashMap::new();

    loop {
        match move_from(position, direction, &map, &mut positions_visited) {
            MovementResult::NewPosition {
                new_position,
                new_direction,
            } => {
                position = new_position;
                direction = new_direction;
            }
            MovementResult::OffTheMap => break,
            MovementResult::LoopFound => todo!(),
        }
    }

    println!(
        "Number of positions visited by the guard on original map: {}",
        positions_visited.len()
    );

    // Second star, release build recommended!
    let possible_placements = generate_possible_obstacle_placements(&map);
    println!(
        "Possible positions at which an obstacle can be placed: {}",
        possible_placements.len()
    );

    println!("Searching for placements resulting in a loop...");
    let mut loop_placements = 0;
    'outer: for map in possible_placements {
        let mut position = map.initial_position;
        let mut direction = map.initial_direction.clone();
        let mut positions_visited: HashMap<Position, Vec<Direction>> = HashMap::new();

        loop {
            match move_from(position, direction, &map, &mut positions_visited) {
                MovementResult::NewPosition {
                    new_position,
                    new_direction,
                } => {
                    position = new_position;
                    direction = new_direction;
                }
                MovementResult::OffTheMap => break,
                MovementResult::LoopFound => {
                    loop_placements += 1;
                    break;
                }
            }
        }
    }

    println!(
        "Number of possible loop placements found: {}",
        loop_placements
    );
}
