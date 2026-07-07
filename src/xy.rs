use crate::xyz::Xyz;
use crate::{LinearRgb, Rgb};

/// CIE 1931 xy chromaticity coordinates.
///
/// The `x` and `y` channels are stored as unsigned 16-bit fixed-point values
/// scaled across `0..=u16::MAX`. Convert them back to normalized chromaticity
/// coordinates by dividing each channel by `u16::MAX`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Xy {
    x: u16,
    y: u16,
}

impl Xy {
    const MULTIPLIER: f32 = 65_535.0;

    /// Creates xy chromaticity coordinates from fixed-point channels.
    ///
    /// The constructor does not validate whether the coordinate is inside a
    /// specific device gamut.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Returns the fixed-point `x` chromaticity coordinate.
    #[must_use]
    pub const fn x(self) -> u16 {
        self.x
    }

    /// Returns the fixed-point `y` chromaticity coordinate.
    #[must_use]
    pub const fn y(self) -> u16 {
        self.y
    }
}

/// Converts XYZ tristimulus values into fixed-point xy chromaticity.
///
/// A zero-intensity XYZ color maps to `(0, 0)`.
impl From<Xyz> for Xy {
    fn from(xyz: Xyz) -> Self {
        let sum = xyz.x() + xyz.y() + xyz.z();

        if sum == 0.0 {
            Self::new(0, 0)
        } else {
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            Self::new(
                ((xyz.x() / sum) * Self::MULTIPLIER).round_ties_even() as u16,
                ((xyz.y() / sum) * Self::MULTIPLIER).round_ties_even() as u16,
            )
        }
    }
}

/// Converts linear RGB into fixed-point xy chromaticity through XYZ.
impl From<LinearRgb> for Xy {
    fn from(linear_rgb: LinearRgb) -> Self {
        Self::from(Xyz::from(linear_rgb))
    }
}

/// Converts gamma-encoded RGB into fixed-point xy chromaticity through linear
/// RGB and XYZ.
impl From<Rgb> for Xy {
    fn from(rgb: Rgb) -> Self {
        Self::from(Xyz::from(rgb))
    }
}
