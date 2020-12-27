fn main() {
    // panic!("crash and burn");

    // panic in a library; set $env:RUST_BACKTRACE=1
    let v = vec![1, 2, 3];

    v[99];
}
