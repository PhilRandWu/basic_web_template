use std::sync::Mutex;
use crate::dao::db_context::Database;
use std::sync::Arc;
pub mod config;
pub mod model;
pub mod controller;
pub mod dao;

// AppState
// This the primary dependency for our application's dependency injection.
// Each controller_test function that interacts with the database will require an `AppState` instance in
// order to communicate with the database.
pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>
}