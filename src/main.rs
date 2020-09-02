use pls_args::RuntimeArguments;

// Entry point for the pls program
fn main() {
    let opt = RuntimeArguments::gather();
    println!("Debug: {}", opt.is_debug());
}
