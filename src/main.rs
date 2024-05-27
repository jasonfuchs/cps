use std::{ffi::c_uint, path::PathBuf};

use clap::Parser;
use cps::prelude::*;
use diesel::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    addr: String,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(short = 'i', long)]
    ds: Option<c_uint>,
    #[arg(short, long)]
    sh_cp: Option<c_uint>,
    #[arg(short = 'l', long)]
    st_cp: Option<c_uint>,
    #[arg(short, long)]
    db: Option<String>,
    #[arg(short = 'f', long)]
    dev: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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

    loop {
        let path = PathBuf::from("/sys/bus/w1/devices")
            .join(
                args.dev
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or("10-00080253aa82"),
            )
            .join("temperature");

        let file = sh_reg
            .get_ref()
            .file_open(path, pigpiod_if2::PI_FILE_READ)?;

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
                .get_result(&mut conn)?;

            println!("{:?}", row);
        }
    }
}
