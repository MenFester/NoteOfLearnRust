fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores2);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:#?}", score);

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);    // 不覆盖

    println!("{:?}", scores3);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", map);
}
