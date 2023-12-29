use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

pub struct Group {
    id: u64,
    name: String,
}

impl<'c> FromRow<'c, MySqlRow> for Group {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Group {
            id: row.get(0),
            name: row.get(1)
        })
    }
}