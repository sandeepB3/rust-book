// We Learn About:

// Variables and Mutability
// Constants
// Shadowing

// Data Types
// There are two data type subsets: scalar and compound
// Scalar types: integers, floating-point numbers, Booleans, and characters.
// Compound types: tuples and arrays.
// Arrays are allocated on the stack

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    //Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    println!("\nThe value of const in global scope is: {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("\nThe value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');

    // masking x
    let x = five();
    println!("\nThe value of x is: {x}");

}

// In function signatures, you must declare the type of each parameter.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("\nThe measurement is: {value}{unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// Functions with Return Values
fn five() -> i32 {
    5
}





