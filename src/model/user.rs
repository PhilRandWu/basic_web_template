use super::Group;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

pub struct User {
    id: String,
    name: String,
    email: String,
    groups: Vec<Group>,
}

impl<'c> FromRow<'c, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            groups: Vec::with_capacity(0),
        })
    }
}