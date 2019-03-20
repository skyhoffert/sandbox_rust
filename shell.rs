/*
 * Sky Hoffert
*/

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    println!("Oxidizer Shell Started.");

    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        match arg.as_ref() {
            "-r" => println!("ARGH"),
            _ => (),
        }
    }

    let mut input = String::new();

    loop {
        print!("{} > ", "os");
        std::io::stdout().flush().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).ok();

        match input.trim() {
            "exit" => break,
            "ls" => {
                let paths = fs::read_dir("./").unwrap();

                for path in paths {
                    print!("{}  ", path.unwrap().path().display());
                }
                println!("");
            },
            _ => (),
        }
    }

    println!("Exiting Oxidizer Shell.");
}

