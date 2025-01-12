use std::{fmt, str::FromStr, time::Duration};

use bitflags::bitflags;
use inherited::InheritedTimingPoint;
use uninherited::UninheritedTimingPoint;

use crate::Error;

use super::sample_set::SampleSet;

pub mod inherited;
pub mod uninherited;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Effects: u8 {
        const KIAI = 1 << 0;
        const EMIT_FIRST_BAR_LINE = 1<<3;
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimingPoint {
    Uninherited(UninheritedTimingPoint),
    Inherited(InheritedTimingPoint),
}

impl TimingPoint {
    #[inline]
    pub const fn time(&self) -> Duration {
        match self {
            TimingPoint::Uninherited(t) => t.time,
            TimingPoint::Inherited(t) => t.time,
        }
    }

    #[inline]
    pub const fn slider_velocity(&self) -> f32 {
        match self {
            TimingPoint::Uninherited(_) => 1.0,
            TimingPoint::Inherited(t) => t.slider_velocity,
        }
    }

    #[inline]
    pub const fn sample_set(&self) -> Option<SampleSet> {
        match self {
            TimingPoint::Uninherited(t) => t.sample_set,
            TimingPoint::Inherited(t) => t.sample_set,
        }
    }

    #[inline]
    pub const fn sample_index(&self) -> Option<i32> {
        match self {
            TimingPoint::Uninherited(t) => t.sample_index,
            TimingPoint::Inherited(t) => t.sample_index,
        }
    }

    #[inline]
    pub const fn volume(&self) -> i32 {
        match self {
            TimingPoint::Uninherited(t) => t.volume,
            TimingPoint::Inherited(t) => t.volume,
        }
    }

    #[inline]
    pub const fn effects(&self) -> Effects {
        match self {
            TimingPoint::Uninherited(t) => t.effects,
            TimingPoint::Inherited(t) => t.effects,
        }
    }

    #[inline]
    pub const fn uninherited(&self) -> bool {
        matches!(self, TimingPoint::Uninherited(_))
    }

    #[inline]
    pub const fn inherited(&self) -> bool {
        matches!(self, TimingPoint::Inherited(_))
    }
}

impl FromStr for TimingPoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);

        let time = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected time while parsing timing point",
            ))
            .and_then(|s| s.parse().map_err(Error::from))
            .map(Duration::from_millis)?;

        let beat_length = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected beat length while parsing timing point",
            ))
            .and_then(|s| s.parse::<f32>().map_err(Error::from))?;

        let meter = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected meter while parsing timing point",
            ))
            .and_then(|s| s.parse::<i32>().map_err(Error::from))?;

        let sample_set = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected sample set while parsing timing point",
            ))
            .and_then(|s| s.parse::<i32>().map_err(Error::from))
            .and_then(|i| {
                Ok(match i {
                    0 => None, // beatmap default
                    _ => Some(SampleSet::try_from(i)?),
                })
            })?;

        let sample_index = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected sample index while parsing timing point",
            ))
            .and_then(|s| s.parse::<i32>().map_err(Error::from))
            .map(|i| match i {
                0 => None,
                _ => Some(i),
            })?;

        let volume = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected volume while parsing timing point",
            ))
            .and_then(|s| s.parse::<i32>().map_err(Error::from))?;

        let uninherited = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected uninherited while parsing timing point",
            ))
            .and_then(|s| s.parse::<u8>().map_err(Error::from))
            .map(|i| i == 1)?;

        let effects = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected effects while parsing timing point",
            ))
            .and_then(|s| s.parse::<u8>().map_err(Error::from))
            .map(Effects::from_bits_truncate)?;

        Ok(if uninherited {
            TimingPoint::Uninherited(UninheritedTimingPoint {
                time,
                beat_length,
                meter,
                sample_set,
                sample_index,
                volume,
                effects,
            })
        } else {
            let slider_velocity = beat_length / -25.0;

            TimingPoint::Inherited(InheritedTimingPoint {
                time,
                slider_velocity,
                sample_set,
                sample_index,
                volume,
                effects,
            })
        })
    }
}

impl fmt::Debug for TimingPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimingPoint::Uninherited(t) => t.fmt(f),
            TimingPoint::Inherited(t) => t.fmt(f),
        }
    }
}
