use self::coefficients::Coefficients;
use crate::{LinearRgb, Rgb};

mod coefficients;

/// CIE 1931 XYZ tristimulus values.
///
/// `Xyz` is the intermediate color space used when converting RGB colors into
/// xy chromaticity. Components are stored as floating-point tristimulus values
/// and are not clamped by the constructor.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz {
    x: f32,
    y: f32,
    z: f32,
}

impl Xyz {
    const X_COEFFICIENTS: Coefficients = Coefficients::new(0.664_511, 0.154_324, 0.162_028);
    const Y_COEFFICIENTS: Coefficients = Coefficients::new(0.283_881, 0.668_433, 0.047_685);
    const Z_COEFFICIENTS: Coefficients = Coefficients::new(0.000_088, 0.072_310, 0.986_039);

    /// Creates XYZ tristimulus values.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the X tristimulus value.
    #[must_use]
    pub const fn x(self) -> f32 {
        self.x
    }

    /// Returns the Y tristimulus value.
    #[must_use]
    pub const fn y(self) -> f32 {
        self.y
    }

    /// Returns the Z tristimulus value.
    #[must_use]
    pub const fn z(self) -> f32 {
        self.z
    }
}

/// Converts linear RGB into XYZ by applying the crate's RGB-to-XYZ matrix.
impl From<LinearRgb> for Xyz {
    fn from(linear_rgb: LinearRgb) -> Self {
        Self {
            x: Self::X_COEFFICIENTS.apply(linear_rgb),
            y: Self::Y_COEFFICIENTS.apply(linear_rgb),
            z: Self::Z_COEFFICIENTS.apply(linear_rgb),
        }
    }
}

/// Converts gamma-encoded RGB into XYZ through linear RGB.
impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        Self::from(LinearRgb::from(rgb))
    }
}
