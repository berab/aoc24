use std::fs::read_to_string;

fn solve_p1(first_numbers: &mut Vec<i32>, second_numbers: &mut Vec<i32>) -> u32 {
    first_numbers.sort();
    second_numbers.sort();
    let distance_sum: u32 = first_numbers.into_iter().zip(second_numbers)
        .map(|(a, b)| (*a-*b).abs() as u32).sum();
    distance_sum
}

fn solve_p2(first_numbers: Vec<i32>, second_numbers: Vec<i32>) -> u32 {
    let occurances: u32 = first_numbers.into_iter()
        .map(|a| (a as u32)*cal_occurance_count(a, &second_numbers) as u32).sum();
    occurances
}

fn cal_occurance_count(num: i32, nums: &Vec<i32>) -> usize {
    let occurance_count: usize = nums.iter().filter(|a| **a == num).count();
    occurance_count
}

pub fn main() -> (u32, u32) {
    let (mut first_numbers, mut second_numbers): (Vec<i32>, Vec<i32>) = read_to_string("inputs/day1.txt")
        .unwrap().lines()
        .map(|line| {
            let mut numbers = line.split_whitespace();
            let a = numbers.next().unwrap().parse::<i32>().unwrap();
            let b = numbers.next().unwrap().parse::<i32>().unwrap();
            (a, b)
        }).collect();
    
    (solve_p1(&mut first_numbers, &mut second_numbers), 
     solve_p2(first_numbers, second_numbers))
}
