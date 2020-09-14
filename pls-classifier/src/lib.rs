mod configuration;
mod lookup;

use configuration::Configuration;

pub fn init() {
    let _conf = Configuration::new();
}
