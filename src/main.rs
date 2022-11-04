#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("starting up");

    let mut x = 0;

    for i in 1..10 {
        println!("At {}", i);
        x += i;
    };


    if x > 10 {
        println!("So much!");
    }

    info!("Goodbye!");
}
