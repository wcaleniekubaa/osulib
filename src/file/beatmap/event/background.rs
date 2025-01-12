use std::{path::PathBuf, str::FromStr};

use nalgebra_glm::IVec2;

use crate::Error;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Background {
    /// Offset in osu! pixels from the centre of the screen.
    /// For example, an offset of `50,100` would have the background shown 50 osu! pixels to the right and 100 osu! pixels down from the centre of the screen.
    /// If the offset is `0,0`, writing it is optional.
    pub offset: IVec2,
    /// Location of the background image relative to the beatmap directory.
    pub filename: PathBuf,
}

impl FromStr for Background {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);

        let _ = parts.next().ok_or(Error::InvalidData(
            "expected type while parsing background event",
        ))?; // ignore the type

        let _ = parts.next().ok_or(Error::InvalidData(
            "expected start time while parsing background event",
        ))?;

        let filename = parts.next().ok_or(Error::InvalidData(
            "expected filename while parsing background event",
        ))?;
        let filename = PathBuf::from(filename);

        let x = parts.next().map(str::parse);
        let y = parts.next().map(str::parse);
        let offset = IVec2::new(
            if let Some(x) = x { x? } else { 0 },
            if let Some(y) = y { y? } else { 0 },
        );

        Ok(Self { offset, filename })
    }
}
