//! Color space conversions for RGB, linear RGB, CIE 1931 XYZ, and CIE 1931 xy
//! coordinates.
//!
//! The crate stores gamma-encoded RGB values as [`Rgb`], converts them to
//! linear-light values with [`LinearRgb`], projects those values into [`Xyz`],
//! and finally exposes chromaticity coordinates through [`Xy`].
//!
//! [`Xy`] uses unsigned 16-bit fixed-point channels scaled over the full
//! `0..=u16::MAX` range. This representation is useful for protocols and device
//! APIs that exchange xy chromaticity as two compact integer channels.
//!
//! # Examples
//!
//! Convert a gamma-encoded RGB color directly to xy coordinates:
//!
//! ```
//! use bunt::{Rgb, Xy};
//!
//! let xy = Xy::from(Rgb::RED);
//!
//! assert_eq!(xy, Xy::new(0xB35A, 0x4C9F));
//! ```
//!
//! Inspect the intermediate linear RGB representation:
//!
//! ```
//! use bunt::{LinearRgb, Rgb};
//!
//! let linear = LinearRgb::from(Rgb::new(128, 64, 32));
//!
//! assert!(linear.red() > linear.green());
//! assert!(linear.green() > linear.blue());
//! ```

pub use self::linear_rgb::LinearRgb;
pub use self::rgb::Rgb;
pub use self::xy::Xy;
pub use self::xyz::Xyz;

mod linear_rgb;
mod rgb;
mod xy;
mod xyz;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_xy_white() {
        let rgb = Rgb::new(255, 255, 255);
        let xy: Xy = rgb.into();
        assert_eq!(xy, Xy::new(0x529E, 0x543B));
    }

    #[test]
    fn test_rgb_to_xy_red() {
        let rgb = Rgb::new(255, 0, 0);
        let xy: Xy = rgb.into();
        assert_eq!(xy, Xy::new(0xB35A, 0x4C9F));
    }

    #[test]
    fn test_rgb_to_xy_green() {
        let rgb = Rgb::new(0, 255, 0);
        let xy: Xy = rgb.into();
        assert_eq!(xy, Xy::new(0x2C23, 0xBF2D));
    }

    #[test]
    fn test_rgb_to_xy_blue() {
        let rgb = Rgb::new(0, 0, 255);
        let xy: Xy = rgb.into();
        assert_eq!(xy, Xy::new(0x22B0, 0x0A35));
    }
}
