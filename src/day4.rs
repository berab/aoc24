use std::fs::read_to_string;


fn look_for_xmas(input: &String, line_idx: usize) -> u32 {
    let mut xmas_count_line: u32 = 0;
    for (char_idx, cur_char) in input.lines().nth(line_idx).unwrap().chars().enumerate() {
        if cur_char == 'X' {
            xmas_count_line += look_for_mas(input, line_idx, char_idx);
        }
    }
    xmas_count_line
}

fn look_for_mas(input: &String, line_idx: usize, char_idx: usize) -> u32{
    let mut xmas_count: u32 = 0;
    let line_len = input.lines().count() as usize;
    let char_len = input.lines().nth(0).unwrap().chars().count() as usize;

    // Checking every corner
    for i in 0..3 { // Line pos.
        for j in 0..3 { // Char pos.
            let direction = (i, j);
            let (new_indices, legal) = cal_new_indices(line_idx, char_idx, direction);
            if legal == false || new_indices.0 >= line_len || 
               new_indices.1 >= char_len {
               continue;
            }
            if input.lines().nth(new_indices.0).unwrap().chars().nth(new_indices.1).unwrap() == 'M' {
                if look_for_as(input, new_indices.0, new_indices.1, direction) {
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

fn look_for_as(input: &String, line_idx: usize, char_idx: usize, direcition: (usize, usize)) -> bool {
    let (mut new_indices, mut legal) = cal_new_indices(line_idx, char_idx, direcition);
    let line_len = input.lines().count() as usize;
    let char_len = input.lines().nth(0).unwrap().chars().count() as usize;
    if legal == false || new_indices.0 >= line_len || 
       new_indices.1 >= char_len {
        return false
    }
    if input.lines().nth(new_indices.0).unwrap().chars().nth(new_indices.1).unwrap() == 'A' {
        (new_indices, legal) = cal_new_indices(new_indices.0, new_indices.1, direcition);
        if legal == false || new_indices.0 >= line_len || 
           new_indices.1 >= char_len {
           return false
        }
        if input.lines().nth(new_indices.0).unwrap().chars().nth(new_indices.1).unwrap() == 'S' {
            return  true
        }
    }
    false
}

fn cal_new_indices(line_idx: usize, char_idx: usize, direcition: (usize, usize)) -> ((usize, usize), bool) {
    let (mut new_line_idx, mut new_char_idx): (usize, usize) = (0, 0);
    let mut legal = true;
    match direcition.0 {
        0 => {
            if line_idx > 0 {
                new_line_idx = line_idx - 1;
            } else {
                legal = false;
            }
        }
        1 => new_line_idx = line_idx,
        2 => new_line_idx = line_idx + 1,
        _ => println!("Direction error"),
    }
    match direcition.1 {
        0 => {
            if char_idx > 0 {
                new_char_idx = char_idx - 1;
            } else {
                legal = false;
            }
        }
        1 => new_char_idx = char_idx,
        2 => new_char_idx = char_idx + 1,
        _ => println!("Direction error"),
    }
    ((new_line_idx, new_char_idx), legal)
}

fn look_for_x_mas(input: &String, line_idx: usize) -> u32 {
    let mut xmas_count_line: u32 = 0;
    let line_len = input.lines().count() as usize;
    let char_len = input.lines().nth(0).unwrap().chars().count() as usize;

    for (char_idx, cur_char) in input.lines().nth(line_idx).unwrap().chars().enumerate() {
        if line_idx == 0 || char_idx == 0 ||
            line_idx >= line_len - 1|| char_idx >= char_len - 1 {
            continue;
        }
        if cur_char == 'A' {
            xmas_count_line += look_for_cross(input, line_idx, char_idx);
        }
    }
    xmas_count_line
}

fn look_for_cross(input: &String, line_idx: usize, char_idx: usize) -> u32{
    let mut x_count: u32 = 0;
    let mut first_indices: (usize, usize);
    let mut second_indices: (usize, usize);

    // Checking every corner
    for i in 0..2 {
        if i == 0 {
            first_indices = (line_idx - 1, char_idx - 1);
            second_indices = (line_idx + 1, char_idx + 1);
        } else {
            first_indices = (line_idx - 1, char_idx + 1);
            second_indices = (line_idx + 1, char_idx - 1);
        }

        let first_char =  input.lines().nth(first_indices.0).unwrap().chars().nth(first_indices.1).unwrap();
        let second_char =  input.lines().nth(second_indices.0).unwrap().chars().nth(second_indices.1).unwrap();
        if (first_char, second_char) == ('M', 'S') || 
            (first_char, second_char) == ('S', 'M') {
                x_count += 1
        }
    }
    if x_count == 2 {
        return 1
    }
    0
}

fn solve_p1(input: &String) -> u32 {
    let mut total_xmas_count: u32 = 0;
    for i in 0..input.lines().count() {
        total_xmas_count += look_for_xmas(input, i);
    }
    total_xmas_count
}

fn solve_p2(input: &String) -> u32 {
    let mut total_x_mas_count: u32 = 0;
    for i in 0..input.lines().count() {
        total_x_mas_count += look_for_x_mas(input, i);
    }
    total_x_mas_count
}

pub fn main() -> (u32, u32) {
    let input: String = read_to_string("inputs/day4.txt").unwrap();
    (solve_p1(&input), 
     solve_p2(&input))
}
