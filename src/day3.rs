use std::fs::read_to_string;
use regex::Regex;

fn mul(input: &String) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let res: Vec<_> = re.find_iter(input)
        .map(|cap| {
            let cap_splitted: Vec<&str> = cap.as_str().split(',').collect();
            let a: i32 = cap_splitted[0][4..].to_string().parse::<i32>().unwrap();
            let str_b: &str = cap_splitted[1];
            let len_b: usize = str_b.len();
            let b: i32 = str_b[..len_b - 1].to_string().parse::<i32>().unwrap();
            a*b
        })
        .collect();
    let sum: i32 = res.into_iter().sum();
    sum
}

fn solve_p1(input: &String) -> i32 {
    mul(input)
}

fn solve_p2(input: &String) -> i32 {
    let (mut total_sum, mut mul_active) = (0, true);
    for cur_split in input.split("don't()") {
        for cur_str in cur_split.split("do()") {
            if mul_active {
                total_sum += mul(&(cur_str.to_string()));
            }
            mul_active = true;
        }
        mul_active = false;
    }
    total_sum
}

pub fn main() -> (i32, i32) {
    let input: String = read_to_string("inputs/day3.txt").unwrap();
    (solve_p1(&input), 
     solve_p2(&input))
}
