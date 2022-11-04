mod error;
mod release;

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

fn foo() {
    println!("foo!");
}

fn faz() {
    bar(4);
    println!("faz: {}", 22);
}

fn bar(x: i32) {
    let y = 7;
    let z = x + y;
    println!("z: {}", z);
}
