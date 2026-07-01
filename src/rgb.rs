/// An RGB color.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    /// Plain red.
    pub const RED: Self = Self::new(0xff, 0x00, 0x00);

    /// Plain green.
    pub const GREEN: Self = Self::new(0x00, 0xff, 0x00);

    /// Plain blue.
    pub const BLUE: Self = Self::new(0x00, 0x00, 0xff);

    /// Cyan (aka aqua).
    pub const CYAN: Self = Self::new(0x00, 0xff, 0xff);

    /// Magenta.
    pub const MAGENTA: Self = Self::new(0xff, 0x00, 0xff);

    /// Yellow.
    pub const YELLOW: Self = Self::new(0xff, 0xff, 0x00);

    /// White.
    pub const WHITE: Self = Self::new(0xff, 0xff, 0xff);

    /// Black (kinda...).
    pub const BLACK: Self = Self::new(0x00, 0x00, 0x00);

    /// Create a new `Rgb` color.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Get the red component.
    #[must_use]
    pub const fn red(self) -> u8 {
        self.red
    }

    /// Get the green component.
    #[must_use]
    pub const fn green(self) -> u8 {
        self.green
    }

    /// Get the blue component.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.blue
    }
}
