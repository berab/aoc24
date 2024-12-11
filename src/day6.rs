use std::collections::HashMap;
use std::fs::read_to_string;

fn solve_p1(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    while map.walk_or_turn() { }
    Some(map.path_traveled())
}

fn solve_p2(input: &str) -> Option<u32> {
    let mut loop_count: u32 = 0;
    let mut map = Map::from(input);
    let (rows, cols) = (map.cols, map.rows);
    let start_location = map.location;

    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| (row as isize, col as isize) != start_location)
        .for_each(|(row, col)| {
            map = Map::from(input);
            map.points[row][col] = b'O';
            while map.walk_or_turn() { }
            if map.in_loop {
                println!("{:?}", (row, col));
                loop_count += 1;
            }
        });
    Some(loop_count)
}

#[derive(Clone)]
struct Map {
    points: Vec<Vec<u8>>,
    location: (isize, isize),
    rows: usize,
    cols: usize,
    dirs: Vec<u8>, // up, down, right, left
    turn_map: HashMap<u8, u8>,
    in_loop: bool,
    saw_guard: bool,
}

impl Map {
    fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .points
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .unwrap_or(&b'.')
    }

    fn walk(&mut self, row: usize, col: usize) {
        match (self.points[row][col]) {
            94 => self.location.0 -= 1, // UP
            118 => self.location.0 += 1, // DOWN
            62 => self.location.1 += 1, // RIGHT
            60 => self.location.1 -= 1, // LEFT
            _ => (),
        }
    }

    fn print_location(&self) {
        println!("{:?}", self.location);
    }

    fn path_traveled(&self) -> u32 {
        let mut count: u32 = 0;
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| self.points[row][col] == b'X')
            .for_each(|(row, col)| count += 1);
        count
    }
    
    fn walk_or_turn(&mut self) -> bool {
        let (row, col) = (self.location.0 as usize, self.location.1 as usize);
        // self.print_location();
        // println!("");
        self.walk(row, col);
        // self.print_location();
        // println!("");
        let (next_row, next_col) = (self.location.0 as usize, self.location.1 as usize);
        match self.points.get(next_row).and_then(|row| row.get(next_col)) {
            Some(&v) => {
                // println!("{}", v);
                if v == b'#' || v == b'O' { // Barrier
                    if self.saw_guard && v == b'O' {
                        // println!("{}", v);
                        self.in_loop = true;
                        return false;
                    }
                    if v == b'O' {
                        // println!("{}", v);
                        self.saw_guard = true;
                    }
                    self.location = (row as isize, col as isize); // Reset location
                    // println!("{:?}", self.location);
                    // println!("{:?}", self.points[row][col]);
                    self.points[row][col] = self.turn_map[&self.points[row][col]]; // Turn 90deg
                    self.walk_or_turn()
                } else {
                    // println!("prev: {:?}", (row, col));
                    // println!("next: {:?}", self.location);
                    // println!("{:?}", self.points[row][col]);
                    self.points[next_row][next_col] = self.points[row][col];
                    self.points[row][col] = b'X';
                    return true
                }
            },
            None => {
                self.points[row][col] = b'X';
                return false
            }, // Can't walk anymore.
        }
    }

    fn put_guard(&mut self) {
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .for_each(|(row, col)| {
                self.points[row][col] = b'O';
            });

    }

}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let points: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();
        let (rows, cols) = (points.len(), points.first().map_or(0, |row| row.len()));
        let dirs: Vec<u8> = vec![94, 118, 62, 60];
        let location: (usize, usize) = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| dirs.contains(&points[row][col]))
            .next().unwrap();

        let mut turn_map: HashMap<u8, u8> = HashMap::new();
        turn_map.insert(94, 62);
        turn_map.insert(62, 118);
        turn_map.insert(118, 60);
        turn_map.insert(60, 94);

        let in_loop: bool = false;
        let saw_guard: bool = false;

        Self { points, location: (location.0 as isize, location.1 as isize), rows, cols , dirs, turn_map, in_loop, saw_guard }
    }
}

pub fn main() {
    let input: String = read_to_string("inputs/day6_test.txt").unwrap();
    // let input: String = read_to_string("inputs/day6.txt").unwrap();
    print!("Day 5 | Puzzle 1: {}, Puzzle 2: {}.", solve_p1(&input).unwrap(), solve_p2(&input).unwrap());
}
