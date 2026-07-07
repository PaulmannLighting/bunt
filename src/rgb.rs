/// Gamma-encoded 8-bit RGB color.
///
/// Each channel is stored in the conventional `0..=255` range. Conversions from
/// this type first linearize the channels before projecting them into XYZ or xy
/// color spaces.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    /// Full-intensity red.
    pub const RED: Self = Self::new(0xff, 0x00, 0x00);

    /// Full-intensity green.
    pub const GREEN: Self = Self::new(0x00, 0xff, 0x00);

    /// Full-intensity blue.
    pub const BLUE: Self = Self::new(0x00, 0x00, 0xff);

    /// Full-intensity cyan, also known as aqua.
    pub const CYAN: Self = Self::new(0x00, 0xff, 0xff);

    /// Full-intensity magenta.
    pub const MAGENTA: Self = Self::new(0xff, 0x00, 0xff);

    /// Full-intensity yellow.
    pub const YELLOW: Self = Self::new(0xff, 0xff, 0x00);

    /// Full-intensity white.
    pub const WHITE: Self = Self::new(0xff, 0xff, 0xff);

    /// Zero-intensity black.
    pub const BLACK: Self = Self::new(0x00, 0x00, 0x00);

    /// Creates an RGB color from red, green, and blue channels.
    ///
    /// Each channel is an 8-bit gamma-encoded value.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Returns the red channel.
    #[must_use]
    pub const fn red(self) -> u8 {
        self.red
    }

    /// Returns the green channel.
    #[must_use]
    pub const fn green(self) -> u8 {
        self.green
    }

    /// Returns the blue channel.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.blue
    }
}
