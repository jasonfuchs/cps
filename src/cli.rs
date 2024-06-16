use std::num;

use clap::*;
use cps::prelude::*;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Address of pigpio daemon")]
    pub address: String,
    #[arg(short, long)]
    #[arg(help = "Port of pigpio daemon")]
    #[arg(default_value = "8888")]
    pub port: String,
    #[arg(short, long)]
    #[arg(help = "Input pin of shift register")]
    #[arg(default_value = "17")]
    pub input: Gpio,
    #[arg(short, long)]
    #[arg(help = "Shift pin of shift register")]
    #[arg(default_value = "21")]
    pub shift: Gpio,
    #[arg(short, long)]
    #[arg(help = "Latch pin of shift register")]
    #[arg(default_value = "27")]
    pub latch: Gpio,
    #[arg(short, long)]
    #[arg(help = "SQLite 3 database URL")]
    #[arg(default_value = ".sqlite.db")]
    pub url: String,
    #[arg(short, long)]
    #[arg(help = "DS18S20 sensor ID")]
    #[arg(default_value = "10-00080253aa82")]
    pub device: String,
    #[arg(short, long)]
    #[arg(help = "Stop after <COUNT> requests")]
    pub count: Option<num::NonZeroUsize>,
    #[arg(short, long)]
    #[arg(help = "Output format")]
    #[arg(default_value = "txt")]
    pub format: Format,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Format {
    #[value(name = "txt")]
    PlainText,
    #[value(name = "csv")]
    CommaSeperatedValues,
}
