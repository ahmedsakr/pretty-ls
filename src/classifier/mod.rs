mod configuration;
mod lookup;
pub use lookup::ConfigurationLookup;

pub fn init() -> ConfigurationLookup {
    ConfigurationLookup::new()
}
