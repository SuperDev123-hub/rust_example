fn main() {
    let a: [i32; 5] = [1, 2, 3, -5, -2];
    let mut b = a.map(|item| item.abs());
    b.sort();
    println!("{:?}", b);
    println!("{:?}", a);
}
