use std::str::FromStr;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleSet {
    #[default]
    Normal = 1,
    Soft = 2,
    Drum = 3,
}

impl SampleSet {
    /// Parses a string slice into a [`SampleSet`], returning [`None`] if the string is not one of the
    /// following:
    ///
    /// - "Normal": [`Normal`][SampleSet::Normal]
    /// - "Soft": [`Soft`][SampleSet::Soft]
    /// - "Drum": [`Drum`][SampleSet::Drum]
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }

    /// Parses an i32 into a SampleSet, returning None if the value is not one of the following:
    ///
    /// - 1: [`Normal`][SampleSet::Normal]
    /// - 2: [`Soft`][SampleSet::Soft]
    /// - 3: [`Drum`][SampleSet::Drum]
    pub fn from_i32(value: i32) -> Option<Self> {
        Self::try_from(value).ok()
    }
}

impl FromStr for SampleSet {
    type Err = Error;

    /// Attempts to convert a string slice into a SampleSet.
    ///
    /// This function will return [Err(Error::InvalidInput)][Error::InvalidInput] if the string is
    /// not one of the following:
    ///
    /// - "Normal": [`Normal`][SampleSet::Normal]
    /// - "Soft": [`Soft`][SampleSet::Soft]
    /// - "Drum": [`Drum`][SampleSet::Drum]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(SampleSet::Normal),
            "Soft" => Ok(SampleSet::Soft),
            "Drum" => Ok(SampleSet::Drum),
            _ => Err(Error::InvalidInput("invalid sample set value")),
        }
    }
}

impl TryFrom<i32> for SampleSet {
    type Error = Error;

    /// Attempts to convert an i32 into a SampleSet.
    ///
    /// This function will return [Err(Error::InvalidInput)][Error::InvalidInput] if the value is
    /// not one of the following:
    ///
    /// - 1: [`Normal`][SampleSet::Normal]
    /// - 2: [`Soft`][SampleSet::Soft]
    /// - 3: [`Drum`][SampleSet::Drum]
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SampleSet::Normal),
            2 => Ok(SampleSet::Soft),
            3 => Ok(SampleSet::Drum),
            _ => Err(Error::InvalidInput("invalid sample set value")),
        }
    }
}
