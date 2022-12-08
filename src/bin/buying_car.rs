const VALUE_DECREASE_RATE: f64 = 0.005;
fn main() {
    println!("---------");
}

fn nb_months(old: i32, new: i32, saving: i32, percent: f64) -> (i32, i32) {
    let mut old = old as f64;
    let mut new = new as f64;
    let mut month = 0;
    let mut savings = 0.0;
    let mut percent = 1.0 - (percent / 100.0);
    while old + savings < new {
        month += 1;
        if month % 2 == 0 {
            percent -= VALUE_DECREASE_RATE;
        }

        old *= percent;
        new *= percent;
        savings += saving as f64;
    }
    (month, ((old + savings) - new).round() as i32)
}
