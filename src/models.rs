use std::fmt::{Display, Formatter};

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

#[derive(Queryable, Selectable, Debug, PartialEq, PartialOrd)]
#[diesel(table_name = temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Temperature {
    id: i32,
    temperature: f32,
    created_at: NaiveDateTime,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "|{}|{}|{}|", self.id, self.temperature, self.created_at)
    }
}
