use crate::xyz::Xyz;
use crate::{LinearRgb, Rgb};

/// CIE 1931 XY color space representation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Xy {
    x: u16,
    y: u16,
}

impl Xy {
    const MULTIPLIER: f32 = 65_535.0;

    /// Create a new `Xy` color.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Return the X value.
    #[must_use]
    pub const fn x(self) -> u16 {
        self.x
    }

    /// Return the Y value.
    #[must_use]
    pub const fn y(self) -> u16 {
        self.y
    }
}

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

impl From<LinearRgb> for Xy {
    fn from(linear_rgb: LinearRgb) -> Self {
        Self::from(Xyz::from(linear_rgb))
    }
}

impl From<Rgb> for Xy {
    fn from(rgb: Rgb) -> Self {
        Self::from(Xyz::from(rgb))
    }
}
