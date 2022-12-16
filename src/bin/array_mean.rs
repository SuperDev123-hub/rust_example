use std::collections::HashMap;

fn solution(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut mean_index: HashMap<String, Vec<i32>> = HashMap::new();

    for (ind, sub_arr) in a.iter().enumerate() {
        let sum: i32 = sub_arr.iter().sum();
        let mean_elements: f32 = (sum as f32) / (sub_arr.len() as f32);
        let index_values = mean_index.get_mut(&mean_elements.to_string()).unwrap();
        index_values.push(ind as i32);
    }

    println!("{:?}", mean_index);
    res
}

fn main() {
    solution(vec![vec![3, 3, 4, 2]])
}
