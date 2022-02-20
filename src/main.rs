use rand::Rng;
use std::io;

fn main() {
    println!("=======================");
    println!("=== Dice calculator ===");
    println!("=======================");
    println!("'Count' must be greater 0");
    println!("'Dice' also must be greater 0");

    // Get Count

    println!("Count:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let count: u16 = input
        .trim()
        .parse()
        .expect("It's not a number");

    // Get Dice

    println!("Dice:");

    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let dice: u16 = input
        .trim()
        .parse()
        .expect("It's not a number");

    if count > 0 && dice > 0 {
        let mut value: u16 = 0;

        for i in 1..=count {
            let current = rand::thread_rng().gen_range(0..=dice);

            value += current;

            println!("{}: 1k{} {}", i, dice, current);
        }

        println!("{}k{} = {}", count, dice, value);
    }
}
