fn main() {

    // Moving Ownership - 4.1
    let first = String::from("Ferris");
    let first_clone = first.clone();        //Cloning Avoids Moves
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    // References (Borrowing) Are Non-Owning Pointers - 4.2
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let _s = format!("{} {}", m1, m2);

    // Dereferencing a Pointer Accesses Its Data - 4.2
    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value, x points to the value 2

    let r1: &Box<i32> = &x;   // r1 points to x on the stack
    let _b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;       // r2 points to the heap value directly
    let _c: i32 = *r2;        // so only one dereference is needed to read it

    // Mutable Reference - 4.2
    let mut s = String::from("hello");
    change(&mut s);

    // Avoiding Simultaneous Aliasing and Mutation - 4.2
    simultaneous();

    // Pointer Safety Principle: data should never be aliased and mutated at the same time.
    // Rust ensures the safety of references through the borrow checker.

    // The core idea behind the borrow checker is that variables have three kinds of permissions on their data:
    // -> Read (R): data can be copied to another location.
    // -> Write (W): data can be mutated in-place.
    // -> Own (O): data can be moved or dropped.
}

// Chap 4.1 Ownerships
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// Chap 4.2 References and Borrowing
fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

// Chap 4.2 References and Borrowing
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Chap 4.2 Simultaneous Aliasing and Mutation
fn simultaneous() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    let x = *num;
    v.push(4);
    println!("Third element is {}", x);
}