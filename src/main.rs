use ansi_term::Colour::{Blue, Red};
use clap::Parser;
use diced::{Args, Die};
use rand::prelude::*;
use regex::Regex;
use std::process;

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
        let mut dice = vec![];
        for die in &args.dice {
            let capture = match_die.captures(&die).unwrap_or_else(|| {
                eprintln!("[ERR] ~> Die entered improperly: {}", die);
                process::exit(1);
            });
            let size: u16 = capture["size"].parse().unwrap_or_else(|_| {
                eprintln!("[ERR] ~> Die size too large: {}", die);
                eprintln!("[ERR] ~> [Limit is {}]", u16::MAX);
                process::exit(1);
            });
            let quantity: u16 = capture["quantity"].parse().unwrap_or_else(|_| {
                eprintln!("[ERR] ~> Die quantity too large: {}", die);
                eprintln!("[ERR] ~> [Limit is {}]", u16::MAX);
                process::exit(1);
            });
            if let Some(modifier) = capture.get(3) {
                let temp_mod: i16 = modifier.as_str().parse().expect("huh?");
                dice.push(Die::new(quantity, size, temp_mod));
            } else {
                dice.push(Die::new(quantity, size, 0));
            };
        }
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
    let rolls: Vec<u16> = vec![0; die.quantity().into()];
    let mut sum: i16 = 0;
    let mut successes: u16 = 0;
    let mut failures: u16 = 0;
    let colored_rolls: Vec<String> = rolls
        .into_iter()
        .map(|_| rng.gen_range(1..=die.size()))
        .inspect(|x| sum += (*x as i16) + die.modifier())
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
fn main() {
    let arguments = Args::parse();
    let mut rng = thread_rng();
    match parse(&arguments) {
        Some(dice) => {
            for die in dice {
                roll_die(&die, &arguments, &mut rng);
                if rng.gen_range(1..=(80 - whoami::username().len() as i32))
                    >= (80 - whoami::username().len() as i32) - 20
                    && !arguments.painless
                {
                    let plural: &str;
                    match die.quantity() {
                        1 => plural = "die",
                        _ => plural = "dice",
                    }
                    println!("Your {plural} rolled off the table. Doesn't count!");
                    roll_die(&die, &arguments, &mut rng);
                }
            }
        }
        None => {
            panic!("oh!");
        }
    }
}
