fn solve(a: usize, b: usize) -> (usize, usize) {
    if a == 0 || b == 0 {
        return (a, b);
    }
    if a >= 2 * b {
        return solve(a - 2 * b, b);
    }
    if b >= 2 * a {
        return solve(a, b - 2 * a);
    }
    return (a, b);
}

fn main() {
    println!("{:?}", solve(6, 19));
}
