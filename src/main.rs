mod sys;
mod classifier;
mod args;

use args::RuntimeArguments;

// Entry point for the pls program
fn main() {
    let classifier = classifier::init();

    let opt = RuntimeArguments::gather();
    println!("Debug mode: {}", opt.is_debug());

    let output = sys::list_dir(opt.directory()).expect("Unable to list directory files");

    for file in &output {
        print!("{}{}  ", classifier.get_color(file), file);
    }

    println!();
}
