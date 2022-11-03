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
