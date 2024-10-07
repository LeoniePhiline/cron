#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::needless_doctest_main)]
//! A cron expression parser and schedule explorer built with jiff.
//!
//! # Example
//!
//! ```
//! use jiff_cron::Schedule;
//! use jiff::tz::TimeZone;
//! use std::str::FromStr;
//!
//! fn main() {
//!   //               sec  min   hour   day of month   month   day of week   year
//!   let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
//!   let schedule = Schedule::from_str(expression).unwrap();
//!   println!("Upcoming fire times:");
//!   for datetime in schedule.upcoming(TimeZone::UTC).take(10) {
//!     println!("-> {}", datetime);
//!   }
//! }
//!
//! /*
//! Upcoming fire times:
//! -> 2018-06-01 09:30:00 UTC
//! -> 2018-06-01 12:30:00 UTC
//! -> 2018-06-01 15:30:00 UTC
//! -> 2018-06-15 09:30:00 UTC
//! -> 2018-06-15 12:30:00 UTC
//! -> 2018-06-15 15:30:00 UTC
//! -> 2018-08-01 09:30:00 UTC
//! -> 2018-08-01 12:30:00 UTC
//! -> 2018-08-01 15:30:00 UTC
//! -> 2018-08-15 09:30:00 UTC
//! */
//! ```

/// Error types used by this crate.
pub mod error;

mod ordinal;
mod parsing;
mod queries;
mod schedule;
mod specifier;
mod time_unit;

pub use crate::schedule::{OwnedScheduleIterator, Schedule, ScheduleIterator};
pub use crate::time_unit::TimeUnitSpec;
