use crate::Rgb;

/// Linear RGB color representation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearRgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgb {
    /// Create a new `LinearRgb` color.
    #[must_use]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    /// Get the red component.
    #[must_use]
    pub const fn red(self) -> f32 {
        self.red
    }

    /// Get the green component.
    #[must_use]
    pub const fn green(self) -> f32 {
        self.green
    }

    /// Get the blue component.
    #[must_use]
    pub const fn blue(self) -> f32 {
        self.blue
    }
}

impl From<Rgb> for LinearRgb {
    fn from(rgb: Rgb) -> Self {
        Self {
            red: correct_channel(rgb.red()),
            green: correct_channel(rgb.green()),
            blue: correct_channel(rgb.blue()),
        }
    }
}

/// Apply gamma correction to a single channel.
#[inline]
fn correct_channel(channel: u8) -> f32 {
    let channel_f = f32::from(channel) / 255.0;
    if channel_f <= 0.04045 {
        channel_f / 12.92
    } else {
        ((channel_f + 0.055) / 1.055).powf(2.4)
    }
}
