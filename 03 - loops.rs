// loops in Rust 
// a counter that count 1 to 10
// for loops in Rust
fn main() {
    for i in 1..=10 {
        println!("{}", i);
    }
}

// while loops in Rust
fn main() {
    let mut i = 1;
    while i <= 10 {
        println!("{}", i);
        i += 1;
    }
}
// loop loops in Rust
fn main() {
    let mut i = 1;
    loop {
        println!("{}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }
}
