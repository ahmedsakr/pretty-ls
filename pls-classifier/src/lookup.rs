use crate::configuration::Configuration;

pub struct Lookup {
    config: Configuration,
}

impl Lookup {
    pub fn new() -> Self {
        Self {
            config: Configuration::new(),
        }
    }
}
