use self::coefficients::Coefficients;
use crate::{LinearRgb, Rgb};

mod coefficients;

/// X-Y-Z color space.
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

    /// Create a new `Xyz` color.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Return the X value.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Return the Y value.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Return the Z value.
    #[must_use]
    pub const fn z(&self) -> f32 {
        self.z
    }
}

impl From<LinearRgb> for Xyz {
    fn from(linear_rgb: LinearRgb) -> Self {
        Self {
            x: Self::X_COEFFICIENTS.apply(linear_rgb),
            y: Self::Y_COEFFICIENTS.apply(linear_rgb),
            z: Self::Z_COEFFICIENTS.apply(linear_rgb),
        }
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        Self::from(LinearRgb::from(rgb))
    }
}
