mod args;
mod classifier;
mod sys;

use args::RuntimeArguments;

// Entry point for the pls program
fn main() {
    let classifier = classifier::init();

    let opt = RuntimeArguments::gather();
    let output = sys::list_dir(opt.directory()).expect("Unable to list directory files");

    for file in &output {
        print!("{}{}  ", classifier.get_color(file), file);
    }

    println!();
}
