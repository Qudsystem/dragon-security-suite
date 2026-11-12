// Event parser module
// Responsible for parsing JSON events from Tetragon with zero-copy deserialization

pub mod models;
pub mod event_parser;

pub use models::*;
pub use event_parser::EventParser;
