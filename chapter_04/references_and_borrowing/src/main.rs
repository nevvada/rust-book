// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // wouldnt work because we're returning a reference here. we would need to return the value
// }

// fn main() {
//     let mut s = String::from("hello");

//     let s1 = &s;
//     let s2 = &s;
//     println!("{s1}, {s2}");
//     let s3 = &mut s;
//     s3.push_str("...World!");
//     println!("{s3}")

//     // let mut s = String::from("hello");
//     // {
//     //     let my_string = &mut s;
//     // }

//     // change(&mut s);

//     // println!("{s}, {s1}, {s2}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("{s1}");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
