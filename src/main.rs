mod args;

// Entry point for the pls program
fn main() {
    let opt = args::RuntimeOptions::gather();
    println!("Debug: {}", opt.is_debug());
}
