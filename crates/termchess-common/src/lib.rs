use std::error::Error;

pub type TResult<T> = Result<T, Box<dyn Error>>;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Command line chess, with artificial intelligence
pub struct Args {}
