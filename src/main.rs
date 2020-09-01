mod args;

use args::RuntimeOptions;

// Entry point for the pls program
fn main() {
    let opt = RuntimeOptions::gather();
    println!("Debug: {}", opt.is_debug());
}
