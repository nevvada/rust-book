// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let red = black.0;
//     let origin = Point(0, 0, 0);
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("nevvada"),
//         email: String::from("someone@gmail.com"),
//         sign_in_count: 2,
//     };

//     let user2 = User {
//         email: String::from("anotherexample@gmail.com"),
//         ..user1
//     };

//     user1.email = String::from("newemail@gmail.com");

//     println!("{:?}", user1);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
