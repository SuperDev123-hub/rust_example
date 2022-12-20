fn generate_shape(n: i32) -> String {
    let mut ret = String::new();
    (0..n).for_each(|_| {
        ret.push_str(format!("{}\n", "+".repeat(n as usize)).as_str());
    });

    ret.trim().to_owned()
}
fn main() {
    println!("{}", generate_shape(5));
}
