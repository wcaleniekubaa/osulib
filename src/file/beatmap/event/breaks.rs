use std::{str::FromStr, time::Duration};

use crate::Error;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Break {
    /// Start time of the break, in milliseconds from the beginning of the beatmap's audio.
    pub start_time: Duration,
    /// End time of the break, in milliseconds from the beginning of the beatmap's audio.
    pub end_time: Duration,
}

impl FromStr for Break {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let _ = parts.next().ok_or(Error::InvalidData(
            "expected type while parsing break event",
        ))?; // ignore the type
        let start_time = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected start time while parsing break event",
            ))?
            .parse()
            .map(Duration::from_millis)?;
        let end_time = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected end time while parsing break event",
            ))?
            .parse()
            .map(Duration::from_millis)?;

        Ok(Self {
            start_time,
            end_time,
        })
    }
}
