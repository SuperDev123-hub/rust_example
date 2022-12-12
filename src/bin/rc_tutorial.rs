use std::rc::Rc;

fn main() {
    let shared = Rc::new("aaaaaaaaaaaa".to_string());
    let shared1 = shared.clone();

    let str1 = "abc".to_string();
    let str2 = str1.clone();
    println!("{:?}", shared1);
    println!("{:?}", Rc::strong_count(&shared));
}
