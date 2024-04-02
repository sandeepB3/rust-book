fn main() {

    // Data Must Outlive All Of Its References
    let str = return_a_string();
    println!("{str}")
}

// Error 1
// fn return_a_string() -> &String {
//     let s = String::from("Hello, world!");
//     &s
// }

// Error 1: Fix 1
// fn return_a_string() -> String {
//     let s = String::from("Hello, world!");
//     s
// }

// Error 1: Fix 2
fn return_a_string() -> &'static str {
    "Hello, world!"
}