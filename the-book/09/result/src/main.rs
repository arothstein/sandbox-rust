use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

/* More seasoned example using closures (Chapter 13)

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

*/

/* using unwrap and expect

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    expect, which is similar to unwrap, lets us also choose the panic! error message.
    Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
}


*/

/* Chaining method calls after the ? operator; can be used with functions that have a return type of Result

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}


*/

/* Using fs::read_to_string instead of opening and then reading the file

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

*/
