/// Represents an RGB color.
// TODO: alpha?
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    /// red from 0-255
    pub red: u8,

    /// blue from 0-255
    pub green: u8,

    /// green from 0-255
    pub blue: u8,
}

impl Color {}
