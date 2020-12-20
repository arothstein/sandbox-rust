fn main() {
    // Create a vector
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // Update a vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // vectors are destroyed when they go out of scope.

    // access vector values using either indexing syntax or the get method

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate over vector with for loop
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    // can also iterate over mutable references
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        // * is the dereference operator to get the value in i
        *i += 50;
    }

    // Defining an enum to store values of different types in one vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
