
// enums give you a way of saying a value is one of a possible set of values.
// For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.
// To do this, Rust allows us to encode these possibilities as an enum.

// IP address can be either a version four or a version six address
enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // Instances of each of the two variants of IpAddrKind
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // Gives error -> no implementation for `i8 + Option<i8>`
    //let sum = x + y;

    // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
    match y {
        Some(value) => {
            println!("Value is {}", x+value);
            // Code to handle Some(T) variant
            // Here, you can use the inner value 'value'
        },
        None => {
            println!("No value present");
            // Code to handle None variant
        },
    }

}

fn route(_ip_kind: IpAddrKind) {}

// enum Option<T> {
//     None,
//     Some(T),
// }