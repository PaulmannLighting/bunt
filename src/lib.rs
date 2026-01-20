//! A library for color space conversions between RGB, Linear RGB, XYZ, and xy color spaces.

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
