// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("i: {}, item: {}", i, item);
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut my_string = String::from("hello world");

    let word = first_word_slice(&my_string);

    my_string.clear();
    println!("the first word is: {}", word);
}
