use std::fs::read_to_string;

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut safety: bool = true;
    let rate: bool = levels[0] < levels[1];
    for i in 0..levels.len() - 1 {
        let cur_rate = levels[i] < levels[i+1];
        if (levels[i] - levels[i+1]).abs() > 3 || 
            cur_rate != rate || levels[i] == levels[i+1] {
            safety = false;
        }
    }
    return safety
}

fn is_safe_dampener(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut reduced_levels = levels.clone();
        reduced_levels.remove(i);
        if is_safe(&reduced_levels) {
            return true;
        }
    }
    false
}

fn solve_p1(input: &String) -> u32 {
    let mut safety_count: u32 = 0;
    for str_levels in input.lines() {
        let cur_levels: Vec<i32> = str_levels.split_whitespace()
            .map(|a| a.parse::<i32>().unwrap()).collect();
        if is_safe(&cur_levels) {
            safety_count += 1;
        }
    }
    safety_count
}

fn solve_p2(input: &String) -> u32 {
    let mut safety_count: u32 = 0;
    for line in input.lines() {
        let cur_levels: Vec<i32> = line.split_whitespace()
            .map(|a| a.parse::<i32>().unwrap()).collect();
        let mut safety = is_safe(&cur_levels);
        if safety == false {
            safety = is_safe_dampener(&cur_levels);
        }
        if safety {
            safety_count += 1;
        }
    }
    safety_count
}

pub fn main() -> (u32, u32) {
    let input: String = read_to_string("inputs/day2.txt").unwrap();
    (solve_p1(&input), 
     solve_p2(&input))
}
