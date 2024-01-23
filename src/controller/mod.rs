pub mod index_controller;
pub mod log;

use std::sync::Mutex;
use log::info;
pub use index_controller::init as index_controller_init;
