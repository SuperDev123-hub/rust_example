use std::num;

fn solution(numbers: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for item in (0..numbers.len() - 2) {
        match (
            numbers.get(item),
            numbers.get(item + 1),
            numbers.get(item + 2),
        ) {
            (Some(&a1), Some(&a2), Some(&a3)) if a1 < a2 && a2 > a3 => res.push(1),
            (Some(&a1), Some(&a2), Some(&a3)) if a1 > a2 && a2 < a3 => res.push(1),
            _ => res.push(0),
        }
    }
    res
}

fn main() {}
