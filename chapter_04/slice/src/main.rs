fn main() {
    let s = String::from("hello world");

    first_word(&s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..=3];
    assert_eq!(slice, &[2, 3, 4]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[..=4];
//     let world = &s[6..];
//     let hello_world_1 = &s[..s.len()];
//     let hello_world_2 = &s[..];

//     println!("{} {}", hello, world);
//     println!("{hello_world_1} {hello_world_2}");
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);
//     s.clear();

//     println!("{}", word);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
