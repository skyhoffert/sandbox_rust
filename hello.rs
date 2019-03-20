/*
 * Sky Hoffert
*/

use std::env;

fn main() {
    println!("Hello, Rusty World!");

    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        match arg.as_ref() {
            "-r" => println!("ARGH"),
            _ => (),
        }
    }

    println!("Enter an integer value: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let value: u32 = input.trim().parse().unwrap();
    println!("Got value: {}", value);

    println!("See Ya!");
}
