use rand::Rng;
use std::{io, thread};
use std::time::Duration;
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
    println!("Enter formula without whitespaces and parentheses: (Count)k(Dice)(+-)...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let cond = Regex::new(r"^(\d+k\d+[-+]?)+$").unwrap().is_match(input.trim());

    if cond {
        let formula = input.trim();

        let mut value: i16 = 0;

        if formula.find(['+', '-']).is_some() {
            let mut start: usize = 0;

            while start < formula.len() {
                let index = match formula[start..].find(['+', '-']) {
                    None => formula.len(),
                    Some(num) => start + num
                };
                let current = calc_k(&formula[start..index]);

                let mut operator = "+";

                if start != 0 {
                    operator = &formula[start - 1..start];
                }

                if operator == "+" {
                    value += current;
                } else {
                    value -= current;
                }

                start = index + 1;
            }
        } else {
            value = calc_k(formula);
        }

        if value < 0 {
            value = 0;
        }

        println!("{} = {}", formula, value);
    } else {
        println!("This is not a formula");
        println!("Please see the instructions");
    }

    println!("=======================");

    thread::sleep(Duration::from_secs(5));
}

fn calc_k(string: &str) -> i16 {
    let mut value = 0;
    let collect: Vec<&str> = string.split('k').collect();

    let count: i16 = collect[0]
        .trim()
        .parse()
        .expect("This is not a number");

    let dice: i16 = collect[1]
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

    value
}