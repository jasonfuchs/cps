use cps::prelude::*;
use diesel::prelude::*;

fn main() -> anyhow::Result<()> {
    let sh_reg = ShiftRegister::<4>::builder()
        .addr("rpi07")
        .ds(17)
        .sh_cp(22)
        .st_cp(27)
        .build()?;

    let mut conn = SqliteConnection::establish("./diesel.db")?;

    loop {
        let file = sh_reg.get_ref().file_open(
            "/sys/bus/w1/devices/10-00080253aa82/temperature",
            pigpiod_if2::PI_FILE_READ,
        )?;

        let temp = file
            .read::<16>()?
            .chars()
            .filter(|&c| c != '\n')
            .collect::<String>()
            .parse::<f32>()?
            / 1000.0;

        let width = 4 - temp.to_string().chars().take_while(|&c| c != '.').count();

        sh_reg.display(format!("{:.width$}", temp, width = width))?;

        {
            use cps::schema::temperatures::dsl::*;

            diesel::insert_into(temperatures)
                .values(NewTemperature::from(temp))
                .execute(&mut conn)?;
        }
    }
}
