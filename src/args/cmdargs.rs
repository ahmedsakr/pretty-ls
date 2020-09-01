// This module is intended to abstract away the structopt
// crate logic.

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pretty-ls")]
pub struct PLSOptions {

    /// Turns on debugging
    #[structopt(short, long)]
    pub debug: bool,

    /// Color-codes files by type
    #[structopt(short, long)]
    pub colorcode: bool
}

impl PLSOptions {

    pub fn init() -> PLSOptions {
        PLSOptions::from_args()
    }
}