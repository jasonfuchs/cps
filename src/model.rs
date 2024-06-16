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

#[derive(Selectable, Debug)]
#[diesel(table_name = crate::schema::temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Temperature {
    id: i32,
    temperature: f32,
    created_at: NaiveDateTime,
}

impl Temperature {
    pub fn to_csv(&self) -> String {
        let Self {
            id,
            temperature,
            created_at,
        } = self;
        // convert to seconds (in UTC)
        let created_at = created_at.and_utc().timestamp();
        format!("{},{},{}", id, temperature, created_at)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            id,
            temperature,
            created_at,
        } = self;
        write!(f, "|{:>4}|{:>6.3}|{}|", id, temperature, created_at)
    }
}
