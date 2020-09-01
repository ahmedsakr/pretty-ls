mod cmdargs;

#[derive(Debug)]
pub struct RuntimeOptions {
    options: cmdargs::PLSOptions
}

impl RuntimeOptions {

    // Associated function to derive the runtime options
    // through PLSOptions (which uses structopt)
    pub fn gather() -> RuntimeOptions {
        RuntimeOptions {
            options: cmdargs::PLSOptions::init()
        }
    }

    // User specified debug mode
    pub fn is_debug(&self) -> bool {
        self.options.debug
    }
} 