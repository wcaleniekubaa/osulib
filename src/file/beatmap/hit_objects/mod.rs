pub mod slider;
pub mod spinner;

use std::{fmt, str::FromStr, time::Duration};

use bitflags::bitflags;
use nalgebra_glm::Vec2;
use slider::Slider;
use spinner::Spinner;

use crate::Error;

use super::{hit_sample::HitSample, hit_sound::HitSound};

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HitObjectKind {
    HitCircle,
    Slider(Slider),
    Spinner(Spinner),
}

impl fmt::Debug for HitObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HitObjectKind::HitCircle => write!(f, "HitCircle"),
            HitObjectKind::Slider(slider) => slider.fmt(f),
            HitObjectKind::Spinner(spinner) => spinner.fmt(f),
        }
    }
}

impl HitObjectKind {
    #[inline]
    pub const fn is_slider(&self) -> bool {
        matches!(self, HitObjectKind::Slider(_))
    }

    #[inline]
    pub const fn is_spinner(&self) -> bool {
        matches!(self, HitObjectKind::Spinner(_))
    }

    #[inline]
    pub const fn is_hit_circle(&self) -> bool {
        matches!(self, HitObjectKind::HitCircle)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HitObject {
    /// Position in osu! pixels of the object.
    pub position: Vec2,
    /// Time when the object is to be hit, in milliseconds from the beginning of the beatmap's audio.
    pub time: Duration,
    pub new_combo: bool,
    pub colour_hax: u8,
    /// Bit flags indicating the hitsound applied to the object.
    pub hit_sound: HitSound,
    pub kind: HitObjectKind,
    /// Information about which samples are played when the object is hit.
    pub hit_sample: HitSample,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    struct HitObjectType: u8 {
        const HIT_CIRCLE = 1 << 0;
        const SLIDER = 1 << 1;
        const NEW_COMBO = 1 << 2;
        const SPINNER = 1 << 3;
        const COLOUR_HAX_1 = 1 << 4;
        const COLOUR_HAX_2 = 1 << 5;
        const COLOUR_HAX_3 = 1 << 6;
        const COLOUR_HAX = (1<<4)|(1<<5)|(1<<6);
        const MANIA_HOLD_NOTE = 1 << 7;
    }
}

impl FromStr for HitObject {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let x = parts
            .next()
            .ok_or(Error::InvalidData("expected x while parsing hit object"))?
            .parse()
            .map_err(Error::from)?;
        let y = parts
            .next()
            .ok_or(Error::InvalidData("expected y while parsing hit object"))?
            .parse()
            .map_err(Error::from)?;

        let position = Vec2::new(x, y);

        let time = parts
            .next()
            .ok_or(Error::InvalidData("expected time while parsing hit object"))
            .map(str::parse)
            .map_err(Error::from)?
            .map(Duration::from_millis)?;

        let ty = parts
            .next()
            .ok_or(Error::InvalidData("expected type while parsing hit object"))
            .and_then(|s| s.parse().map_err(Error::from))
            .map(HitObjectType::from_bits_truncate)?;

        let new_combo = ty.contains(HitObjectType::NEW_COMBO);
        let colour_hax = ty & HitObjectType::COLOUR_HAX;
        let colour_hax = colour_hax.bits();
        let colour_hax = colour_hax >> 4;

        let hit_sound = parts
            .next()
            .ok_or(Error::InvalidData(
                "expected hit sound while parsing hit object",
            ))
            .and_then(|s| s.parse().map_err(Error::from))
            .map(HitSound::from_bits_truncate)?;

        let kind = if ty.contains(HitObjectType::HIT_CIRCLE) {
            HitObjectKind::HitCircle
        } else if ty.contains(HitObjectType::SLIDER) {
            HitObjectKind::Slider(Slider::from_parts(&mut parts)?)
        } else if ty.contains(HitObjectType::SPINNER) {
            HitObjectKind::Spinner(Spinner::from_parts(&mut parts)?)
        } else {
            return Err(Error::InvalidData("invalid hit object type"));
        };

        let hit_sample = parts
            .next()
            .map(str::parse)
            .unwrap_or(Ok(HitSample::default()))?;

        Ok(HitObject {
            position,
            time,
            new_combo,
            colour_hax,
            hit_sound,
            kind,
            hit_sample,
        })
    }
}
