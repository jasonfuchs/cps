use std::path;

use anyhow::Result;
use clap::Parser;
use cli::{Args, Format};
use cps::{
    pi::{read_to_string, Pi},
    segment_display::SegmentDisplay,
    shift_register::ShiftRegister,
};
use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};

mod cli;
mod model;
mod schema;

fn main() -> Result<()> {
    let args = Args::parse();

    let pi = Pi::try_with_addr_and_port(&args.address, &args.port)?;

    let sh_reg = ShiftRegister::<4>::builder()
        .pi(&pi)
        .ds(args.input)
        .sh_cp(args.shift)
        .st_cp(args.latch)
        .build()?;

    let mut conn = SqliteConnection::establish(&args.url)?;

    let path = path::PathBuf::from("/sys/bus/w1/devices")
        .join(&args.device)
        .join("temperature");

    // for now no count
    loop {
        let temperature = read_to_string(&pi, &path)?
            .chars()
            .take_while(|&c| c != '\n')
            .collect::<String>()
            .parse::<f32>()?
            / 1000.;

        let width = 4 - temperature
            .to_string()
            .chars()
            .take_while(|&c| c != '.')
            .count();

        sh_reg.write(format!("{temperature:.width$}"))?;

        let row = {
            use model::*;
            use schema::temperatures::dsl::temperatures;

            diesel::insert_into(temperatures)
                .values(NewTemperature::from(temperature))
                .returning(Temperature::as_returning())
                .get_result(&mut conn)?
        };

        match args.format {
            Format::PlainText => println!("{row}"),
            Format::CommaSeperatedValues => println!("{}", row.to_csv()),
        }
    }
}
