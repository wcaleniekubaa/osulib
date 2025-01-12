use std::str::FromStr;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OverlayPosition {
    #[default]
    Below,
    Above,
}

impl OverlayPosition {
    /// Parses a string slice into an [`OverlayPosition`], returning [`None`] if the string is not one of
    /// the following:
    ///
    /// - "Below": [`Below`][OverlayPosition::Below]
    /// - "Above": [`Above`][OverlayPosition::Above]
    #[inline]
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

impl FromStr for OverlayPosition {
    type Err = Error;

    /// Attempts to convert a string slice into an OverlayPosition.
    ///
    /// This function will return [Err(Error::InvalidInput)][Error::InvalidInput] if the string is
    /// not one of the following:
    ///
    /// - "Below": [`Below`][OverlayPosition::Below]
    /// - "Above": [`Above`][OverlayPosition::Above]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Below" => Ok(OverlayPosition::Below),
            "Above" => Ok(OverlayPosition::Above),
            _ => Err(Error::InvalidInput("invalid overlay position value")),
        }
    }
}
