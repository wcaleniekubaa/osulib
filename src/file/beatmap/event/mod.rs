use std::{fmt, str::FromStr};

use background::Background;
use breaks::Break;
use video::Video;

use crate::Error;

pub mod background;
pub mod breaks;
pub mod video;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventKind {
    Background(Background),
    Video(Video),
    Break(Break),
}

impl fmt::Debug for EventKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventKind::Background(b) => b.fmt(f),
            EventKind::Video(v) => v.fmt(f),
            EventKind::Break(b) => b.fmt(f),
        }
    }
}

impl EventKind {
    #[inline]
    pub const fn is_background(&self) -> bool {
        matches!(self, EventKind::Background(_))
    }

    #[inline]
    pub const fn is_video(&self) -> bool {
        matches!(self, EventKind::Video(_))
    }

    #[inline]
    pub const fn is_break(&self) -> bool {
        matches!(self, EventKind::Break(_))
    }
}

impl FromStr for EventKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let ty = parts
            .next()
            .ok_or(Error::InvalidData("expected event type"))?;

        match ty {
            "0" => Ok(EventKind::Background(Background::from_str(s)?)),
            "1" | "Video" => Ok(EventKind::Video(Video::from_str(s)?)),
            "2" | "Break" => Ok(EventKind::Break(Break::from_str(s)?)),
            _ => Err(Error::InvalidData("invalid event type")),
        }
    }
}
