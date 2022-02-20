use rand::Rng;
use std::io;
use regex::Regex;

/*
TODO: add the ability to enter arguments in advance
    example: dicecalc 1k20+1k8+3
*/
fn main() {
    println!("=======================");
    println!("=== Dice calculator ===");
    println!("=======================");
    println!("'Count' must be greater 0");
    println!("'Dice' also must be greater 0");
    println!("So far can only add values");
    println!("=======================");

    println!("Enter formula without whitespaces and parentheses: ^(\\d+k\\d+[-+]*)+$");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let cond = Regex::new(r"^(\d+k\d+[-+]*)+$").unwrap().is_match(input.trim());

    if cond {
        let formula = input.trim();
        let collect: Vec<&str> = input.trim().split(&['+']).collect(); // TODO: add handling minus '-'

        let mut value: u16 = 0;

        println!("input: {}", input);

        for string in collect {
            if !string.is_empty() {
                let collect: Vec<&str> = string.split('k').collect();

                let count: u16 = collect[0]
                    .trim()
                    .parse()
                    .expect("This is not a number");

                let dice: u16 = collect[1]
                    .trim()
                    .parse()
                    .expect("This is not a number");

                if count > 0 && dice > 0 {
                    for _ in 1..=count {
                        let current = rand::thread_rng().gen_range(0..=dice);

                        value += current;

                        println!("1k{} {}", dice, current);
                    }
                }
            }
        }

        println!("{} = {}", formula, value);
    } else {
        println!("This is not a formula");
        println!("Please see the instructions");
    }
    println!("=======================");
}
