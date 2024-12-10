use std::fs;
use std::collections::HashMap;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    fn add(&self, other: &Vector2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Guard {
    direction: Direction,
    position: Vector2,
    path: HashMap<Vector2, Vec<Direction>>
}

impl Guard {
    fn new(position: Vector2, direction: Direction) -> Self {
        Self {
            position,
            direction, 
            path: HashMap::from([(position,vec![direction])])
        }
    }

    fn get_direction(&mut self) -> Vector2 {
        let direction = match &self.direction {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        };
        direction.clone()
    }

    fn next_step(&mut self) -> Vector2 {
        Vector2::new(self.position.x,self.position.y).add(&self.get_direction())
    }

    fn move_step(&mut self) -> () {
        let movement = self.get_direction();
        self.position = self.position.add(&movement);
    }

    fn repeated_pos_and_dir(&mut self, key: &Vector2, value: &Direction) -> bool {
        if let Some(values) = self.path.get(key) {
            values.contains(value)
        } else {
            false
        }
    }

    fn add_to_path(&mut self, key: &Vector2, value: &Direction) -> () {
        self.path.entry(*key).or_insert_with(Vec::new).push(*value);
    }

    fn turn(&mut self) -> () {
        let new_dir = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.direction = new_dir;
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

fn get_start_pos(map: &Vec<Vec<char>>) -> Vector2 {
    let mut start_pos = Vector2::new(0,0);
    for (i,row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '^' {
                start_pos = Vector2::new(j as i32 ,i as i32)
            }
        }
    }
    start_pos 
}

fn in_bounds(x: i32, y: i32, x_lim: i32,y_lim:i32) -> bool {
    let x_in_bound =  (x >= 0) && (x < x_lim);
    let y_in_bound =  (y >= 0) && (y < y_lim);
    x_in_bound && y_in_bound
}

fn walk_map_checking_for_loops(guard: &mut Guard, map: &Vec<Vec<char>>) -> bool {
    let x_lim = map.len() as i32;
    let y_lim = map[0].len() as i32;
    let mut next_pos = guard.next_step();
    
    while in_bounds(next_pos.x, next_pos.y, x_lim, y_lim) {
        log::debug!("before move: x: {}, y: {}, n_steps: {}",guard.position.x, guard.position.y, guard.path.keys().len());
        if map[next_pos.y as usize][next_pos.x as usize] == '#' {
            guard.turn();
            next_pos = guard.next_step();
            log::debug!("TURN! object at x: {}, y: {}", next_pos.x, next_pos.y,);
            continue
        }
        guard.move_step();

        let direction = guard.direction;
        let position = guard.position.clone();
        if guard.repeated_pos_and_dir(&position, &direction) {
            return true
        }
        guard.add_to_path(&position, &direction);
        next_pos = guard.next_step();
        log::debug!("after move: x: {}, y: {}",guard.position.x, guard.position.y);
    }

    return false
}

fn add_obstacles(map: &mut Vec<Vec<char>>, start_pos: Vector2, start_dir: Direction) -> i32 {
    let mut loops = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut guard = Guard::new(start_pos,start_dir);
            if map[i][j] == '#' || ((start_pos.x ,start_pos.y ) == (j as i32, i as i32)) {
                continue;
            }
            map[i][j] = '#';
            if walk_map_checking_for_loops(&mut guard, &map) {
                loops += 1;
            }
            map[i][j] = '.';
        }
    }
    loops
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut lab_map = read_matrix(FILE_NAME);
    log::debug!("Loaded lab_map: {:?}", lab_map);

    let start_pos = get_start_pos(&lab_map);
    log::debug!("start x: {}, start y: {}", start_pos.x, start_pos.y);

    let mut guard = Guard::new(start_pos,Direction::Up);

    walk_map_checking_for_loops(&mut guard, &lab_map);
    log::info!("Number of distinct positions was: {}", guard.path.keys().len());

    let n_loops = add_obstacles(&mut lab_map, start_pos, Direction::Up);
    log::info!("Number of loops creatable with one more obstacle was: {}", n_loops);

}
