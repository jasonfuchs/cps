use std::fmt;

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

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            created_at,
            temperature,
        } = self;
        write!(f, "|{}|{:>6.3}|", created_at, temperature)
    }
}
