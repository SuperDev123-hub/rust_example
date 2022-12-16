fn solution(a: Vec<i32>, b: Vec<i32>, k: i32) -> i32 {    
    let mut res = 0;
    let length = a.len();
    for (index, a_value) in a.iter().enumerate() {
        let b_value = b.get(length - index - 1).unwrap();
        let str = format!("{:?}{:?}", a_value, b_value);        
        if str.parse::<i32>().unwrap_or(&k + 1) < k {
            res +=1;
        }
    }
    res
}

fn main(){

}