use std::{path::PathBuf, str::FromStr, time::Duration};

use nalgebra_glm::IVec2;

use crate::Error;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Video {
    /// Offset in osu! pixels from the centre of the screen.
    /// For example, an offset of `50,100` would have the background shown 50 osu! pixels to the right and 100 osu! pixels down from the centre of the screen.
    /// If the offset is `0,0`, writing it is optional.
    pub offset: IVec2,
    /// Start time of the event, in milliseconds from the beginning of the beatmap's audio. For events that do not use a start time, the default is 0.
    pub start_time: Duration,
    /// Location of the video file relative to the beatmap directory.
    pub filename: PathBuf,
}

impl FromStr for Video {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let _ = parts.next().ok_or(Error::InvalidData(
            "expected type while parsing video event",
        ))?; // ignore the type
        let start_time = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected start time while parsing video event",
            ))?
            .parse()
            .map(Duration::from_millis)?;
        let filename = parts.next().ok_or(Error::InvalidData(
            "expected filename while parsing video event",
        ))?;
        let filename = PathBuf::from(filename);

        let x = parts
            .next()
            .ok_or(Error::InvalidData("expected x while parsing video event"))?
            .parse()
            .map_err(Error::from)?;
        let y = parts
            .next()
            .ok_or(Error::InvalidData("expected y while parsing video event"))?
            .parse()
            .map_err(Error::from)?;

        let offset = IVec2::new(x, y);

        Ok(Self {
            offset,
            start_time,
            filename,
        })
    }
}
