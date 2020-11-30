fn main() {
    let s1 = String::from("hello");

    // ampersands are references, and they allow you to refer to some value without taking ownership of it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // references are immutable by default; can be mutable if variable is mutable and we create and accept a mutable reference, &mut
    // mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
    // - The benefit of having this restriction is that Rust can prevent data races at compile time.
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

// We call having references as function parameters borrowing. As in real life, if a person owns something,
// you can borrow it from them. When you’re done, you have to give it back.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
