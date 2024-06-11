use std::{ffi::c_uint, num::NonZeroU32};

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(help = "Address of the pigpiod daemon")]
    pub address: String,

    #[arg(short, long, help = "Pigpiod port", default_value = "8888")]
    pub port: u16,
    #[arg(short, long, help = "Input GPIO pin", default_value = "17")]
    pub input: c_uint,
    #[arg(short, long, help = "Shift GPIO pin", default_value = "21")]
    pub shift: c_uint,
    #[arg(short, long, help = "Latch GPIO pin", default_value = "27")]
    pub latch: c_uint,
    #[arg(short, long, help = "SQLite database URL", default_value = "sqlite.db")]
    pub url: String,
    #[arg(short, long, help = "Sensor file", default_value = "10-00080253aa82")]
    pub device: String,
    #[arg(short, long, help = "Request count")]
    pub count: Option<NonZeroU32>,
    #[arg(short, long, default_value = "txt", value_enum)]
    pub format: Format,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    #[value(name = "txt")]
    PlainText,
    #[value(name = "csv")]
    CommaSeperatedValues,
}
