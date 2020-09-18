mod configuration;
mod lookup;
use lookup::Lookup;

pub fn init() {
    let _lookup = Lookup::new();
}
