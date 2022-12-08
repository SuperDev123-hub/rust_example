fn main(){
    let words = "aa bb cc";
    let ret = words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
    }).collect::<Vec<String>>().join("abc");
    println!("{:?}",ret);
}