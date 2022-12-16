fn solution(pattern: String, source: String) -> i32 {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut result = 0;
    let parsed_source: String = source
        .chars()
        .map(|c| match vowels.contains(&c) {
            true => '0',
            false => '1',
        })
        .collect();

    println!("parsed == {:?}", parsed_source);
    for index in 0..=source.len() - pattern.len() {
        let sub_array: String = parsed_source
            .chars()
            .skip(index)
            .take(pattern.len())
            .collect();
        println!("{:?}", sub_array);
        println!("{:?}", parsed_source);
        if sub_array == pattern {
            result = result + 1;
        }
    }
    result
}

fn main() {
    let res = solution("010".to_string(), "amazing".to_string());
    println!("{:?}", res);
}

//pattern: "010"
//source: "amazing"
