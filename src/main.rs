use pls_args::RuntimeArguments;
use pls_classifier;
use pls_sys;

// Entry point for the pls program
fn main() {
    let classifier = pls_classifier::init();
    println!("color for index.js: {}", classifier.get_color("index.js"));

    let opt = RuntimeArguments::gather();
    println!("Debug mode: {}", opt.is_debug());

    let output = pls_sys::list_dir(opt.directory()).expect("Unable to list directory files");

    for file in &output {
        print!("{}  ", file);
    }

    println!();
}
