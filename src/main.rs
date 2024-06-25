use std::path::PathBuf;
use std::thread;
use std::time::Duration;

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

    let path = PathBuf::from("/sys/bus/w1/devices")
        .join(&args.device)
        .join("temperature");

    let mut i = 0;
    while args.count.map(usize::from).map_or(true, |count| i < count) {
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
            use diesel::result::DatabaseErrorKind;
            use diesel::result::Error;
            use model::*;
            use schema::temperatures::dsl::temperatures;

            fn insert_into(
                new_temperature: NewTemperature,
                conn: &mut SqliteConnection,
            ) -> Result<Temperature, Error> {
                Ok(diesel::insert_into(temperatures)
                    .values(new_temperature)
                    .returning(Temperature::as_returning())
                    .get_result(conn)?)
            }

            match insert_into(NewTemperature::from(temperature), &mut conn) {
                Ok(row) => row,
                Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    thread::sleep(Duration::from_secs(1));
                    insert_into(NewTemperature::from(temperature), &mut conn)?
                }
                Err(err) => Err(err)?,
            }
        };

        match args.format {
            Format::PlainText => println!("{row}"),
            Format::CommaSeperatedValues => println!("{}", row.to_csv()),
        }

        i += 1;
    }

    Ok(())
}
