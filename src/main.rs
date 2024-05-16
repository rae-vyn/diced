use ansi_term::Colour::{Blue, Red};
use clap::Parser;
use diced::{Args, Die, DiceSet, Profile, Config, validate};
use rand::prelude::*;
use regex::Regex;
use std::process;
use confy;

/// DICED
/// A dice roller with no bugs, only features. /j

fn parse(args: &Args) -> Option<Vec<Die>> {
    if let Ok(match_die) =
        /*
        - This Regex Grabs:
            - The quantity of dice to throw
            - The size (side number) of dice to throw
            - An optional modifier that adds/deducts from the die roll
        */
        Regex::new(r"(?m)(?<quantity>\d+)[d\\/](?<size>\d+)(?<modifier>[\+\-]\d+)?")
    {
        let dice = validate(args.dice.clone(), match_die);
        if dice.len() == 0 {
            eprintln!("[ERR] ~> No die passed in.");
            process::exit(1);
        };
        return Some(dice);
    };
    return None;
}

fn die_format(x: &i16, size: &i16, color: bool) -> String {
    let x_as_str = format!("{}", x);
    if !color {
        return x_as_str;
    };
    match x {
        x if x <= &1 => Red.bold().paint(x_as_str).to_string(),
        x if x >= size => Blue.bold().paint(x_as_str).to_string(),
        _ => x_as_str,
    }
}

fn roll_die(die: &Die, arguments: &Args, rng: &mut ThreadRng) -> () {
    if die.modifier() == 0 {
        println!("{}d{}:", die.quantity(), die.size());
    } else {
        let mod_string = format!(
            "{}{}",
            match die.modifier() {
                i if i < 0 => "-",
                _ => "+",
            },
            die.modifier().abs()
        );
        println!("{}d{} {}:", die.quantity(), die.size(), mod_string);
    }
    let pool: Vec<u16> = vec![0; die.quantity().into()];
    let mut successes: u16 = 0;
    let mut failures: u16 = 0;
    let rolls: Vec<u16> = pool.clone()
        .into_iter()
        .map(|_| rng.gen_range(1..=die.size()))
        .collect();
    let sum: u16 = rolls.iter().sum();
    let colored_rolls: Vec<String> = rolls
        .into_iter()
        .inspect(|x| {
            if *x >= die.size() {
                successes += 1
            }
            if *x == 1 {
                failures += 1
            }
        })
        .map(|x| {
            die_format(
                &(die.modifier().wrapping_add(x as i16)),
                &(die.size() as i16),
                arguments.color,
            )
        })
        .collect();
    
    if arguments.sum {
        println!("=> ({}): [{}]", colored_rolls.join(", "), sum);
    } else if arguments.count {
        println!(
            "=> ({}): [crit successes: {}, crit failures: {}]",
            colored_rolls.join(", "),
            successes,
            failures
        );
    } else {
        println!("=> ({})", colored_rolls.join(", "));
    }
}

fn main() -> Result<(), confy::ConfyError>{
    let arguments = Args::parse();
    let mut rng = thread_rng();

    match parse(&arguments) {
        Some(dice) => {
            for die in dice {
                roll_die(&die, &arguments, &mut rng);
                if rng.gen_range(1..=100) > 78 && arguments.painful {
                    let plural: &str;
                    match die.quantity() {
                        1 => plural = "die",
                        _ => plural = "dice",
                    }
                    println!("Your {plural} rolled off the table. Doesn't count!");
                    roll_die(&die, &arguments, &mut rng);
                }
            }
            Ok(())
        }
        None => {
            panic!("oh!");
        }
    }
}
