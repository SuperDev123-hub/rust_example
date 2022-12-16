fn solution(numbers: Vec<i32>, left: i32, right: i32) -> Vec<bool> {
    let mut res = Vec::new();
    for (index, value) in numbers.iter().enumerate() {
        let divided_result = value % (index as i32  + 1);
        match divided_result {
            0 => {
                let focus = (value / (index as i32 + 1)) as i32;
                if focus >= left && focus <= right {
                    res.push(true);
                }else {
                    res.push(false);
                }
            },
            _=> res.push(false)
        }
    }
    res
}

fn main() {

}