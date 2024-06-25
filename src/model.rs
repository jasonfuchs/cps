use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    thread,
    time::Duration,
};

use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTemperature {
    temperature: f32,
}

impl From<f32> for NewTemperature {
    #[inline]
    fn from(temperature: f32) -> Self {
        Self { temperature }
    }
}

impl NewTemperature {
    #[inline]
    pub fn try_insert(self, conn: &mut SqliteConnection) -> QueryResult<Temperature> {
        #[inline]
        fn _try_now(
            temperature: &NewTemperature,
            conn: &mut SqliteConnection,
        ) -> QueryResult<Temperature> {
            diesel::insert_into(crate::schema::temperatures::table)
                .values(temperature)
                .returning(Temperature::as_returning())
                .get_result(conn)
        }

        Ok(
            match _try_now(&self, conn).map_err(|_| {
                thread::sleep(Duration::from_secs(1));
                _try_now(&self, conn)
            }) {
                Ok(row) => row,
                Err(Ok(row)) => row,
                Err(err) => err?,
            },
        )
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Temperature {
    created_at: NaiveDateTime,
    temperature: f32,
}

impl Temperature {
    pub fn to_csv(&self) -> String {
        let Self {
            created_at,
            temperature,
        } = self;
        // convert to seconds (in UTC)
        let created_at = created_at.and_utc().timestamp();
        format!("{},{}", created_at, temperature)
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let Self {
            created_at,
            temperature,
        } = self;
        write!(f, "|{}|{:>6.3}|", created_at, temperature)
    }
}
