use crate::Error;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Spinner {
    /// End time of the spinner, in milliseconds from the beginning of the beatmap's audio.
    pub end_time: Duration,
}

impl Spinner {
    pub fn from_parts<'a>(mut iter: impl Iterator<Item = &'a str>) -> Result<Self, Error> {
        let end_time = iter
            .next()
            .ok_or(Error::InvalidData(
                "expected end time while parsing spinner",
            ))
            .and_then(|s| u64::from_str(s).map_err(Error::from))
            .map(Duration::from_millis)?;

        Ok(Self { end_time })
    }
}
