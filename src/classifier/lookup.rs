use crate::classifier::configuration::Configuration;
use termion::color;

pub struct ConfigurationLookup {
    config: Configuration,
}

// Default color for a file that doesn't have a valid configuration
// rule.
const DEFAULT_COLOR: [u8; 3] = [200, 200, 200];
const DEFAULT_RGB: color::Fg<color::Rgb> = color::Fg(color::Rgb(200, 200, 200));

impl ConfigurationLookup {
    // Constructor for the ConfigurationLookup struct
    pub fn new() -> Self {
        Self {
            config: Configuration::new(),
        }
    }
    // Retrieves the color for the provided file name.
    pub fn get_color(&self, file: &str) -> String {
        if let Ok(color) = self.config.get_value(file) {
            match color {
                Some(hex) => str_to_rgb(hex).to_string(),
                None => DEFAULT_RGB.to_string(),
            }
        } else {
            println!("Invalid config rule for {}", file);
            DEFAULT_RGB.to_string()
        }
    }
}

// Converts a string of the format #XXXXXX
// into a color::Fg object that holds the
// color::Rgb value of that hex string.
fn str_to_rgb(input: &str) -> color::Fg<color::Rgb> {
    let mut rgb = DEFAULT_COLOR;

    if input.len() >= 7 {
        let mut letters = input.chars();

        if let Some('#') = letters.next() {
            (0..rgb.len()).for_each(|i| {
                let byte = format!(
                    "{}{}",
                    letters
                        .next()
                        .expect("Corrupt state while converting color"),
                    letters
                        .next()
                        .expect("Corrupt state while converting color")
                );

                if let Ok(num) = u8::from_str_radix(&byte, 16) {
                    rgb[i] = num;
                }
            });
        }
    }

    color::Fg(color::Rgb(rgb[0], rgb[1], rgb[2]))
}
