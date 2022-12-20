fn code(s: &str) -> String {
    s.chars()
        .map(|ch| {
            let digit = ch.to_string().parse::<i32>().unwrap();
            let binary = format!("{:b}", digit);
            format!("{:0fill$}{binary}", 1, fill = binary.len())
        })
        .collect::<String>()
}

fn decode(s: &str) -> String {
    let mut str = s.to_owned();
    let mut ret = String::new();
    loop {
        if str.len() == 0 {
            break;
        }
        let letter_length = str.find('1').unwrap() + 1;
        let letter: String = str
            .drain(0..letter_length * 2)
            .collect::<String>()
            .drain(letter_length..letter_length * 2)
            .collect::<String>();

        ret.push_str(format!("{:}", isize::from_str_radix(letter.as_str(), 2).unwrap()).as_str());
    }

    ret
}

fn main() {
    println!("{}", code("62"));
    println!("{}", decode(code("62").as_str()));
}
