#![allow(clippy::needless_return)]

fn add(x: i32, y:i32) -> i32 {
    return x + y;
}

fn main() {
    println!("{}", add(42, 13))
}