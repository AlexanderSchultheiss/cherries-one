mod dev;

fn main() {
    println!("Hello, world!");

    let mut x = 0;

    for i in 1..10 {
        println!("At {}", i);
        x += i;
    };


    if x > 9 {
        println!("So much!");
    }
}


fn foo () {
    println!("foo!");
}

fn bar () {
    let x = 4;
    let y = 10;
    let z = x + y;
    println!("z: {}", z);
}
