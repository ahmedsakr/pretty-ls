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
    #[allow(dead_code)]
    pub fn is_debug(&self) -> bool {
        self.options.debug
    }

    // Directory to work on
    pub fn directory(&self) -> Option<&String> {
        self.options.dir.as_ref()
    }
}
