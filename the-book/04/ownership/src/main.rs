fn main() {
    // Declare a mutable String variable (as opposed to a string literal)
    // String is mutable, string literals are not, because literals are stored on the stack, String stored on the heap
    let mut s = String::from("hello");

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap,
    // unknown at compile time, to hold the contents.

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print 'hello, world!'

    // by default, s2 = s1 would move the value ownership from s1 to s2, immediately invalidating s1
    // To create deep copies (copy both stack and heap for String), use clone method
    // This only applies to heap data, not stack data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
// this scope is now over, and s is no longer valid
