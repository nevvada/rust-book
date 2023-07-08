fn main() {
    let s = "hola";
    // take_ownership(s);
    println!("{s}");
}

// fn first_word(s: &String) -> &String {
//     let bytes = s.as_bytes();

//     // for (i, &item) in bytes.iter().enumerate() {
//     //     if item == b' ' {
//     //         return i;
//     //     }
//     // }
// }

// fn other() {
//     // let s_literal_stored_in_binary: &s = 'hello';
//     // let s_stored_in_heap = String::from("henlo");
//     // let s_clone = s_stored_in_heap.clone();

//     // println!("{s_stored_in_heap} {s_clone}");
// }

// fn take_ownership(some_string: &str) {
//     println!("{some_string}")
// }
