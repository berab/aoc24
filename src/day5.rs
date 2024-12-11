use std::collections::HashMap;
use std::fs::read_to_string;

struct Manual {
    rules: Vec<(u32, u32)>,
    prods: Vec<Vec<u32>>,
}

impl Manual {
    fn correct(&self, prod: &Vec<u32>) -> bool {
        let pos_map: HashMap<u32, usize> = prod
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect();
        // println!("{:?}", pos_map);
        self.rules
            .iter()
            .all(|(x, y)| {
                match (pos_map.get(x), pos_map.get(y)) {
                    (Some(idx1), Some(idx2)) => idx1 < idx2,
                    _ => true,
                }
        })
        // println!("rules: {:?}", self.rules);
        // println!("rules: {:?}", x);
    }

    fn fix(&self, mut prod: Vec<u32>) -> Vec<u32> {
        prod.sort_by(|&x, &y| {
            if self.rules.contains(&(x, y)) {
                std::cmp::Ordering::Less
            } else if self.rules.contains(&(y, x)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }); 
        prod
    }
}

impl From<&str> for Manual {
    fn from(input: &str) -> Self {
        let divided_input: Vec<&str> = input.split("\n\n").collect();
        let (rules_input, prods_input): (&str, &str) = (divided_input[0], divided_input[1]);

        let rules: Vec<(u32, u32)> = rules_input.lines()
            .map(|page| { 
                let split: Vec<u32> = page.split('|').map(|p| p.parse().unwrap()).collect();
                (split[0], split[1])
            })
            .collect();
        let prods: Vec<Vec<u32>> = prods_input.lines()
            .map(|page| page.split(',').map(|p| p.parse().unwrap()).collect())
            .collect();
        
        Self { rules, prods }
    }
}

fn solve_p1(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let manual = Manual::from(input);
    // let prod = vec![47, 53];
    // println!("{:?}", manual.prods);
    manual.prods
        .iter()
        .filter(|prod| manual.correct(prod))
        .for_each(|prod| sum += prod[prod.len()/2]);
    // println!("sum: {:?}", sum);
    // manual.correct(&prod);
    Some(sum)
}

fn solve_p2(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let manual = Manual::from(input);

    manual.prods
        .iter()
        .filter(|prod| !manual.correct(prod))
        .for_each(|prod| {
            let fixed_prod = manual.fix(prod.clone());
            sum += fixed_prod[fixed_prod.len()/2];
        });
    // if manual.correct(&prod) == false {
    //     manual.fix(prod);
    // }
    Some(sum)
}

pub fn main() {
    // let input: String = read_to_string("inputs/day5_test.txt").unwrap();
    let input: String = read_to_string("inputs/day5.txt").unwrap();
    print!("Day 5 | Puzzle 1: {}, Puzzle 2: {}.", solve_p1(&input).unwrap(), solve_p2(&input).unwrap());
}
