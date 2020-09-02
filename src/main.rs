use pls_args::RuntimeArguments;
use pls_sys::list_dir;

// Entry point for the pls program
fn main() {
    let opt = RuntimeArguments::gather();
    println!("Debug mode: {}", opt.is_debug());

    let output = list_dir().expect("Unable to list directory files");

    for file in &output {
        print!("{}\t", file);
    }

    println!();
}
