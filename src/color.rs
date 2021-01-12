/// Represents an RGB color.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// red from 0-255
    pub red: u8,

    /// blue from 0-255
    pub green: u8,

    /// green from 0-255
    pub blue: u8,
}

impl Color {
    /// Create a new color from the respective parts
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}
