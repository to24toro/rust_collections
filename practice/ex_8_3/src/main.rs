use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // let team_name = String::from("Blue");

    // let score = scores.get(&team_name);
    // println!("{:?}", score);

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
