use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::process;

/// The die model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Die {
    /// The number of die to roll
    quantity: u16,
    /// The number of sides on the die.
    size: u16,
    /// The number to add/substract to the roll.
    modifier: i16

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
        return Self { quantity, size, modifier };
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

    /// Removes some unwanted 'features'.
    #[arg(long)]
    pub painful: bool
}
