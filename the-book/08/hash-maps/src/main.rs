fn main() {
    // Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

    use std::collections::HashMap;

    let mut scores = HashMap::new(); // declare empty HashMap

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // zip and collect using vector tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores1: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // For owned values like String, the values will be moved into the hash map and the hash map will be the owner of those values.alloc

    // We can get a value out of the hash map by providing its key to the get method
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterate over key/value pair using for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // UPDATING A HASH MAP
    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Insert a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.
}
