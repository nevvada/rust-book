enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // println!("{:?}", &self)
    }
}

struct XY {
    x: i32,
    y: i32,
}

fn main() {
    let b = Message::Write(String::from("haagha"));
    b.call();
    let a = Message::Quit;

    let mut some_number = Some(5);
    some_number = None;
    let some_char = Some('e');

    // let absent_number: Option<i32> = 55;
}

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::01"));
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// struct testStruct {
//     aa: (u32, u32, u32),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     println!("Hello, world!");
// }

// fn route(ip_kind: IpAddrKind) {

// }
