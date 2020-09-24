fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 20);

    for (key, value) in scores {
        println!("Color: {}\nScore: {}\n", key, value);
    }
}
