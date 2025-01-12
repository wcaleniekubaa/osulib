use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    #[default]
    Standard,
    Taiko,
    Catch,
    Mania,
}

impl Mode {
    /// Parses an i32 into a Mode, returning None if the value is not one of the following:
    ///
    /// - 0: [Standard][Mode::Standard]
    /// - 1: [Taiko][Mode::Taiko]
    /// - 2: [Catch][Mode::Catch]
    /// - 3: [Mania][Mode::Mania]
    #[inline]
    pub fn from_i32(value: i32) -> Option<Self> {
        Self::try_from(value).ok()
    }
}

impl TryFrom<i32> for Mode {
    type Error = Error;

    /// Attempts to convert an [`i32`] value into a [`Mode`].
    ///
    /// This function will return [`Err(Error::InvalidInput)`][Error::InvalidInput] if the value is not one of
    /// the following:
    ///
    /// - 0: [`Standard`][Mode::Standard]
    /// - 1: [`Taiko`][Mode::Taiko]
    /// - 2: [`Catch`][Mode::Catch]
    /// - 3: [`Mania`][Mode::Mania]
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Standard),
            1 => Ok(Mode::Taiko),
            2 => Ok(Mode::Catch),
            3 => Ok(Mode::Mania),
            _ => Err(Error::InvalidInput("invalid mode value")),
        }
    }
}
