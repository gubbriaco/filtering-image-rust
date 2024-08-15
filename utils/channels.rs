/// Represents the color channels of an RGB image.
pub enum ChRGB {
    RED,
    GREEN,
    BLUE,
}

impl ChRGB {
    /// Converts the `ChRGB` enum variant to its corresponding channel index.
    ///
    /// # Returns
    ///
    /// The index of the color channel (0 for RED, 1 for GREEN, 2 for BLUE).
    pub fn as_u8(&self) -> u8 {
        match self {
            ChRGB::RED => 0,
            ChRGB::GREEN => 1,
            ChRGB::BLUE => 2,
        }
    }
}
