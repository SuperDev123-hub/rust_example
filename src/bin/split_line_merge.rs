fn main() {
    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let v = text
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .join("  ");
    // assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
    println!("{:?}", v);
}
