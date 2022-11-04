mod dev;
mod error;

fn main() {
    println!("Hello, world!");

    let mut x = 0;

    for i in 1..8 {
        println!("At {}", i);
        x += i;
    }

    if x > 7 {
        println!("So much!");
    }
}
