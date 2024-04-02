
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;

fn main() {

    // Creating a new instance of struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Creating a new instance of mutable struct
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    // Struct building functions
    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // IMP
    // In this example, we can no longer use user1 after creating user4 because the String in the username field of user1 was moved into user4.
    // If we had given user4 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user4.
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs
    // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}