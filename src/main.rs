use std::path::PathBuf;

use clap::Parser;
use cli::{Args, Format};
use cps::prelude::*;
use diesel::prelude::*;

mod cli;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sh_reg = ShiftRegister::<4>::builder()
        .addr(&args.address)
        .port(args.port)
        .ds(args.input)
        .sh_cp(args.shift)
        .st_cp(args.latch)
        .build()?;

    let mut conn = SqliteConnection::establish(&args.url)?;

    let path = PathBuf::from("/sys/bus/w1/devices")
        .join(&args.device)
        .join("temperature");

    let mut i = 0;
    while args.count.map(u32::from).map_or(true, |count| i < count) {
        let file = sh_reg
            .get_ref()
            .file_open(&path, pigpiod_if2::PI_FILE_READ)?;

        let temp = file
            .read::<16>()?
            .chars()
            .take_while(|&c| c != '\n')
            .collect::<String>()
            .parse::<f32>()?
            / 1000.;

        let width = 4 - temp.to_string().chars().take_while(|&c| c != '.').count();

        sh_reg.display(format!("{temp:.width$}"))?;

        let row = {
            use cps::schema::temperatures::dsl::temperatures;

            diesel::insert_into(temperatures)
                .values(NewTemperature::from(temp))
                .returning(Temperature::as_returning())
                .get_result(&mut conn)?
        };

        match args.format {
            Format::PlainText => println!("{row}"),
            Format::CommaSeperatedValues => println!("{}", row.to_csv()),
        }

        i += 1;
    }

    Ok(())
}
