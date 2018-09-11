//!
//! A library to read nginx log files
//!

extern crate regex;
#[macro_use]
extern crate failure;

///
pub mod format;

pub use format::Format;
