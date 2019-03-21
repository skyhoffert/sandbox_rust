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
        let tokens: Vec<&str> = input.trim().split(" ").collect();

        match tokens[0] {
            "exit" => break,
            "ls" => {
                let paths = fs::read_dir("./").unwrap();

                for path in paths {
                    print!("{}  ", path.unwrap().path().display());
                }
                println!("");
            },
            "add" => {
                let sum = tokens[1].parse::<u64>().unwrap() + tokens[2].parse::<u64>().unwrap();
                println!("Sum = {}", sum);
            },
            "slots" => {
                let mut i: i32 = 0;
                loop {
                    print!("{}\r", i%10);
                    if i > 1000000 { break; }
                    i += 1;
                }
                println!();
            },
            _ => println!("No command, \"{}\"", tokens[0]),
        }
    }

    println!("Exiting Oxidizer Shell.");
}

