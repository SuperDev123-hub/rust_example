fn solution(a: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for index in (0..a.len()) {
        let mut new_value: i32 = 0;
        if let Some(&value) = a.get(index - 1) {
            new_value += value;
        }

        if let Some(&piece) = a.get(index) {
            new_value += piece;
        }

        if let Some(&piece) = a.get(index + 1) {
            new_value += piece;
        }
        res.push(new_value);
    }
    res
}

fn main() {}
