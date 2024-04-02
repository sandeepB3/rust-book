
// Refactoring with Tuples
fn main() {
    tuple_exp();
    print!("\n");
    struct_exp();
    print!("\n");

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn tuple_exp() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with Structs: Adding More Meaning
#[derive(Debug)]         // derive -> attribute, Debug -> trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_exp() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println! uses the formatting known as Display
    // `Rectangle struct` doesn't implement `std::fmt::Display`
    // To display struct we use the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
    // The Debug trait enables us to print our struct in a way that is useful for developers
    // Rust does include functionality to print out debugging information
    // so we add the outer attribute #[derive(Debug)] just before the struct definition

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}