use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pretty-ls")]
struct Opt {

    /// Turns on debugging
    #[structopt(short, long)]
    debug: bool

}

// Entry point for the pls program
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
