use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::temperatures;

#[derive(Insertable, Debug, PartialEq, PartialOrd)]
#[diesel(table_name = temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTemperature {
    temperature: f32,
}

impl From<f32> for NewTemperature {
    fn from(value: f32) -> Self {
        Self { temperature: value }
    }
}

impl From<&Temperature> for NewTemperature {
    fn from(value: &Temperature) -> Self {
        Self {
            temperature: value.temperature,
        }
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, PartialOrd)]
#[diesel(table_name = temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Temperature {
    id: i32,
    temperature: f32,
    created_at: NaiveDateTime,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "| {:>4} | {:>6.3} | {} |",
            self.id, self.temperature, self.created_at
        )
    }
}

impl Temperature {
    pub fn to_csv(&self) -> String {
        format!("{},{},{}", self.id, self.temperature, self.created_at)
    }
}
