fn main() {
    let _v_new: Vec<i32> = Vec::new();
    let _v_macro = vec![1, 2, 3];

    let mut v_push = Vec::new();

    v_push.push(5);
    v_push.push(6);
    v_push.push(7);
    v_push.push(8);

    // Borrowing
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Reference
    let mut v_loop = vec![100, 32, 57];
    for n_ref in &mut v_loop {
        // n_ref has type &mut i32
        *n_ref += 50;

    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
