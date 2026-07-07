use crate::LinearRgb;

/// Row coefficients for converting linear RGB into one XYZ component.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Coefficients {
    red: f32,
    green: f32,
    blue: f32,
}

impl Coefficients {
    /// Creates row coefficients for red, green, and blue channels.
    #[must_use]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    /// Applies the coefficients to a linear RGB color.
    #[must_use]
    pub fn apply(&self, linear_rgb: LinearRgb) -> f32 {
        self.apply_rgb(linear_rgb.red(), linear_rgb.green(), linear_rgb.blue())
    }

    /// Apply the coefficients to individual RGB components.
    fn apply_rgb(&self, red: f32, green: f32, blue: f32) -> f32 {
        red.mul_add(self.red, green.mul_add(self.green, blue * self.blue))
    }
}
