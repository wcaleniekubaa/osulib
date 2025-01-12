use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Countdown {
    #[default]
    Normal = 1,
    Half = 2,
    Double = 3,
}

impl Countdown {
    /// Attempts to convert an [`i32`] value into a [`Countdown`].
    ///
    /// This function will return [`None`] if the value is not one of
    /// the following:
    ///
    /// - 1: [`Normal`][Countdown::Normal]
    /// - 2: [`Half`][Countdown::Half]
    /// - 3: [`Double`][Countdown::Double]
    #[inline]
    pub fn from_i32(value: i32) -> Option<Self> {
        Self::try_from(value).ok()
    }
}

impl TryFrom<i32> for Countdown {
    type Error = Error;

    /// Attempts to convert an [`i32`] value into a [`Countdown`].
    ///
    /// This function will return [`Err(Error::InvalidInput)`][Error::InvalidInput] if the value is not one of
    /// the following:
    ///
    /// - 1: [`Normal`][Countdown::Normal]
    /// - 2: [`Half`][Countdown::Half]
    /// - 3: [`Double`][Countdown::Double]
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Countdown::Normal),
            2 => Ok(Countdown::Half),
            3 => Ok(Countdown::Double),
            _ => Err(Error::InvalidInput("invalid countdown value")),
        }
    }
}
