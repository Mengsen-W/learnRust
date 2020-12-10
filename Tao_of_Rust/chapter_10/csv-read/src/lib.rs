//! This is documentation for the 'csv_challenge' lib create
//! 
//! Usage:
//! '''
//!   use csv_challenge::{
//!     Opt,
//!     {load_csv, write_csv},
//!   };
//! '''
mod core;
mod err;
mod opt;

pub use self::opt::Opt;
pun use self::core::{
  read::{load_csv, write_str},
  write::replace_column,
};