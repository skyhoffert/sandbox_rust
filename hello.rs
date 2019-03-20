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

    println!("See Ya!");
}
