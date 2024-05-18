use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::temperatures;

#[derive(Queryable, Selectable, Debug, PartialEq, PartialOrd)]
#[diesel(table_name = temperatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Temperature {
    id: i32,
    temperature: f32,
    created_at: NaiveDateTime,
}

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
