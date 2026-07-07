use crate::Rgb;

const GAMMA_CHANNEL_SCALE: f32 = 255.0;
const LINEAR_SEGMENT_THRESHOLD: f32 = 0.040_45;
const LINEAR_SEGMENT_DIVISOR: f32 = 12.92;
const EXPONENTIAL_SEGMENT_OFFSET: f32 = 0.055;
const EXPONENTIAL_SEGMENT_SCALE: f32 = 1.055;
const EXPONENTIAL_SEGMENT_GAMMA: f32 = 2.4;

/// Linear-light RGB color.
///
/// The components are normalized floating-point channel intensities. Values
/// produced from [`Rgb`] are in the `0.0..=1.0` range after applying the
/// standard sRGB transfer function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearRgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgb {
    /// Creates a linear RGB color from normalized channel intensities.
    ///
    /// The constructor does not clamp values, which allows callers to model
    /// colors outside the display-referred `0.0..=1.0` range when needed.
    #[must_use]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    /// Returns the red channel intensity.
    #[must_use]
    pub const fn red(self) -> f32 {
        self.red
    }

    /// Returns the green channel intensity.
    #[must_use]
    pub const fn green(self) -> f32 {
        self.green
    }

    /// Returns the blue channel intensity.
    #[must_use]
    pub const fn blue(self) -> f32 {
        self.blue
    }
}

/// Converts gamma-encoded 8-bit RGB into normalized linear-light RGB.
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
    let channel_f = f32::from(channel) / GAMMA_CHANNEL_SCALE;
    if channel_f <= LINEAR_SEGMENT_THRESHOLD {
        channel_f / LINEAR_SEGMENT_DIVISOR
    } else {
        ((channel_f + EXPONENTIAL_SEGMENT_OFFSET) / EXPONENTIAL_SEGMENT_SCALE)
            .powf(EXPONENTIAL_SEGMENT_GAMMA)
    }
}
