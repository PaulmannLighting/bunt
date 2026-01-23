/// An RGB color.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
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
