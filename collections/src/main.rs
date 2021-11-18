fn main() {
    //VECTORS
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v2 = vec![1, 2, 3];
    for i in &v2 {
        println!("{}", i);
    }
    println!("Printing Vector {:?}", v);
    println!("Printing Vector {:?}", v2);

    //SRINGS  //
    let mut s = String::new();
    let data = "initial contents";

    let stri = data.to_string();

    // the method also works on a literal directly:
    let sliteral = "initial contents".to_string();
    let mut updated_str = String::from("foo");
    updated_str.push_str("bar");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // HASH MAPS
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(20);
    let score = scores.get(&team_name);

    println!("SCORE for blue team {:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}
