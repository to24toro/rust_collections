// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);

//     println!("The longest string is {}", result);
// }

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }


struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.' ");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}