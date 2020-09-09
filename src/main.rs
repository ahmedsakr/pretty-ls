use pls_args::RuntimeArguments;
use pls_sys;

// Entry point for the pls program
fn main() {
    let opt = RuntimeArguments::gather();
    println!("Debug mode: {}", opt.is_debug());

    let output = pls_sys::list_dir(opt.directory()).expect("Unable to list directory files");

    for file in &output {
        print!("{}  ", file);
    }

    println!();
}
