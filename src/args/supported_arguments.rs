// This module is intended to abstract away the structopt
// crate logic.

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pretty-ls")]
pub struct PLSArguments {
    /// Directory to look into
    pub dir: Option<String>,

    /// Turns on debugging
    #[structopt(short, long)]
    pub debug: bool,

    /// Color-codes files by type
    #[structopt(short, long)]
    pub colorcode: bool,
}

impl PLSArguments {
    pub fn init() -> PLSArguments {
        PLSArguments::from_args()
    }
}
