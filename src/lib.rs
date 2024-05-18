use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::process;
use std::vec;

/// The die model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Die {
    /// The number of die to roll
    quantity: u16,
    /// The number of sides on the die.
    size: u16,
    /// The number to add/substract to the roll.
    modifier: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiceSet {
    name: String,
    dice: Vec<Die>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    ident: String,
    sets: Vec<DiceSet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    profiles: Vec<Profile>,
}

impl ::std::default::Default for DiceSet {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            dice: vec![],
        }
    }
}

impl ::std::default::Default for Profile {
    fn default() -> Self {
        Self {
            ident: "".to_string(),
            sets: vec![],
        }
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { profiles: vec![] }
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "d{}", self.size);
    }
}

impl Die {
    /// Create a new die using a quantity and size.
    pub fn new(quantity: u16, size: u16, modifier: i16) -> Self {
        if size < 1 {
            eprintln!("Improper Die Size {size}");
            process::exit(1);
        };
        return Self {
            quantity,
            size,
            modifier,
        };
    }

    pub fn size(&self) -> u16 {
        return self.size;
    }

    pub fn quantity(&self) -> u16 {
        return self.quantity;
    }

    pub fn modifier(&self) -> i16 {
        return self.modifier;
    }
}

pub fn validate(dice: Vec<String>, match_die: Regex) -> Vec<Die> {
    let mut result: Vec<Die> = vec![];
    for die in dice {
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
            result.push(Die::new(quantity, size, temp_mod));
        } else {
            result.push(Die::new(quantity, size, 0));
        };
    }
    result
}

/// The CLI arguments parser.
#[derive(Parser)]
pub struct Args {
    /// The dice to roll.
    pub dice: Vec<String>,

    /// Color crit. successes and fails.
    #[arg(short, long)]
    pub color: bool,

    /// Sum the rolls of each die.
    #[arg(short, long)]
    pub sum: bool,

    /// Count the number of successes and failures.
    #[arg(long)]
    pub count: bool,

    /// Adds some unwanted 'features'.
    #[arg(long)]
    pub painful: bool,

    /// Use a profile with a given name
    #[arg(long)]
    pub profile: Option<String>,
}
