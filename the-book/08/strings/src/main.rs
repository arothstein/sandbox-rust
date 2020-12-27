fn main() {
    // Strings are UTF-8 encoded

    // Creating a new empty String
    // let mut s1 = String::new();

    // Creating a new String with initial data
    let // data = "initial contents";

    // let s2 = data.to_string();

    // the method also works on a literal directly:
    // let s3 = "initial contents".to_string();

    // or using the String::from function to create a String from a string literal
    // let s4 = String::from("initial contents");

    // String::from and to_string do the same thing, so which you choose is a matter of style.

    //Appending to a string with push_str and push
    // let mut s5 = String::from("foo");
    // s5.push_str("bar");

    // push_str takes a string slice so it doesn't take ownership of the parameter.
    // let mut s6 = String::from("foo");
    // let s7 = "bar";
    // s6.push_str(s7);
    println!("s7 is {}", s7);

    // push method takes a single character as a parameter and adds it to the String.
    let mut s8 = String::from("lo");
    s8.push('l');

    // Concatenation with the + Operator or the format! Macro
    // let s9 = s8 + &s7; // note s1 has been moved here and can no longer be used

    // + operator uses the add method, whose signature looks like:
    // fn add(self, s: &str) -> String {}

    // For more complicated string combining, we can use the format! macro:
    // let s10 = String::from("tic");
    // let s11 = String::from("tac");
    // let s12 = String::from("toe");

    // let s13 = format!("{}-{}-{}", s10, s11, s12);

    // The version of the code using format! is much easier to read and doesn’t take ownership of any of its parameters.
}
