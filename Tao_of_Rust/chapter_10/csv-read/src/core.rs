pub mod read;
pub mod write;
use crate::error::Error;
use std::{
  fs::file,
  io::{Read, Write},
  path::PathBuf,
};
