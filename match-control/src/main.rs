
// Match Controls
enum _Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn _value_in_cents(coin: _Coin) -> u8 {
    match coin {
        _Coin::Penny => 1,
        _Coin::Nickel => 5,
        _Coin::Dime => 10,
        _Coin::Quarter => 25,
    }
}

// Matching with Option<T>

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:#?}", six);
    println!("{:#?}", none);

}
