mod supported_arguments;
use supported_arguments::PLSArguments;

#[derive(Debug)]
pub struct RuntimeArguments {
    options: PLSArguments,
}

impl RuntimeArguments {
    // Associated function to derive the runtime options
    // through PLSArguments (which uses structopt)
    pub fn gather() -> RuntimeArguments {
        RuntimeArguments {
            options: PLSArguments::init(),
        }
    }

    // User specified debug mode
    pub fn is_debug(&self) -> bool {
        self.options.debug
    }
}
