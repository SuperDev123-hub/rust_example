// use regex::Regex;

// fn balance_statement(statement: &str) -> String {
//     if statement.is_empty() {
//         return String::from("Buy: 0 Sell: 0");
//     }

//     let records = statement.split(", ");
//     let regex = Regex::new("^[A-Za-z0-9.]+\\s(0|[1-9][0-9]*)\\s((0|[1-9][0-9]*)\\.\\d+)\\s([SB])$")
//         .unwrap();

//     let mut buy = 0.0;
//     let mut sell = 0.0;
//     let mut invalid: Vec<&str> = vec![];
//     for record in records {
//         if let Some(captures) = regex.captures(record) {
//             let units = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
//             let price = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
//             let action = captures.get(4).unwrap().as_str();

//             let total = units * price;
//             println!("Total: {}", total);
//             if action == "B" {
//                 buy += total;
//             } else {
//                 sell += total;
//             }
//         } else {
//             invalid.push(record);
//         }
//     }
//     println!("Sell: {}", sell);

//     let mistakes = if invalid.is_empty() {
//         String::from("")
//     } else {
//         format!("; Badly formed {}: {} ;", invalid.len(), invalid.join(" ;"))
//     };
//     format!("Buy: {:.0} Sell: {:.0}{}", buy, sell, mistakes)
// }

fn main() {
    
}