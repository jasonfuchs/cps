use std::{ffi::c_uint, num::NonZeroU32, path::PathBuf};

use cps::prelude::*;
use diesel::prelude::*;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(name = "ADDRESS")]
    addr: String,
    #[arg(short = 'p', long = "port", name = "PORT")]
    port: Option<u16>,
    #[arg(short = 'i', long = "input", name = "INPUT_PIN")]
    ds: Option<c_uint>,
    #[arg(short = 's', long = "shift", name = "SHIFT_PIN")]
    sh_cp: Option<c_uint>,
    #[arg(short = 'l', long = "latch", name = "LATCH_PIN")]
    st_cp: Option<c_uint>,
    #[arg(short = 'u', long = "url", name = "DATABASE_URL")]
    db: Option<String>,
    #[arg(short = 'd', long = "device", name = "DEVICE")]
    dev: Option<String>,
    #[arg(short = 'c', long = "count", name = "COUNT")]
    count: Option<NonZeroU32>,
    #[arg(short = 'f', long = "format", name = "FORMAT", value_enum)]
    format: Option<Format>,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum Format {
    #[value(name = "txt")]
    PlainText,
    #[value(name = "csv")]
    CommaSeperatedValues,
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    let sh_reg = ShiftRegister::<4>::builder()
        .addr(&args.addr)
        .port(args.port.unwrap_or(8888))
        .ds(args.ds.unwrap_or(17))
        .sh_cp(args.sh_cp.unwrap_or(22))
        .st_cp(args.st_cp.unwrap_or(27))
        .build()?;

    let mut conn = SqliteConnection::establish(
        args.db
            .as_ref()
            .map(String::as_str)
            .unwrap_or("./diesel.db"),
    )?;

    let path = PathBuf::from("/sys/bus/w1/devices")
        .join(
            args.dev
                .as_ref()
                .map(String::as_str)
                .unwrap_or("10-00080253aa82"),
        )
        .join("temperature");

    #[inline]
    fn _loop(
        args: &Args,
        sh_reg: &ShiftRegister<4>,
        conn: &mut SqliteConnection,
        path: &PathBuf,
    ) -> anyhow::Result<()> {
        let file = sh_reg
            .get_ref()
            .file_open(&path, pigpiod_if2::PI_FILE_READ)?;

        let temp = file
            .read::<16>()?
            .chars()
            .take_while(|&c| c != '\n')
            .collect::<String>()
            .parse::<f32>()?
            / 1000.0;

        let width = 4 - temp.to_string().chars().take_while(|&c| c != '.').count();

        sh_reg.display(format!("{:.width$}", temp, width = width))?;

        {
            use cps::schema::temperatures::dsl::*;

            let row = diesel::insert_into(temperatures)
                .values(NewTemperature::from(temp))
                .returning(Temperature::as_returning())
                .get_result(conn)?;

            match args.format.unwrap_or(Format::PlainText) {
                Format::PlainText => println!("{}", row),
                Format::CommaSeperatedValues => println!("{}", row.to_csv()),
            }
        }

        Ok(())
    }

    match args.count {
        None => loop {
            _loop(&args, &sh_reg, &mut conn, &path)?;
        },
        Some(count) => Ok(for _ in 0..count.into() {
            _loop(&args, &sh_reg, &mut conn, &path)?
        }),
    }
}
