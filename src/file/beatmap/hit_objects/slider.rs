use std::str::FromStr;

use nalgebra_glm::Vec2;

use crate::{
    file::beatmap::{hit_sound::HitSound, sample_set::SampleSet},
    iter::{AndThenExt, OkMapExt},
    Error,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Curve {
    /// Bézier curves of arbitrary degree can be made.
    /// Multiple bézier curves can be joined into a single slider by repeating their points of intersection.
    Bezier(Vec<Vec2>),
    /// Catmull curves are an interpolating alternative to bézier curves.
    /// They are rarely used today due to their lack of visual appeal.
    CatmullRom(Vec<Vec2>),
    /// These curves form a straight path between all of their points.
    Linear(Vec<Vec2>),
    /// Perfect circle curves are limited to three points (including the hit object's position) that define the boundary of a circle.
    /// Using more than three points will result in the curve type being switched to bézier.
    Perfect(Vec<Vec2>),
}

impl FromStr for Curve {
    type Err = Error;

    /// Attempts to convert a string slice into a Curve.
    ///
    /// The format for a curve is as follows: `type|x1,y1|x2,y2|...|xN,yN`.
    ///
    /// The type is a single character:
    ///
    /// - `B`: Bézier curve
    /// - `C`: Catmull curve
    /// - `L`: Linear curve
    /// - `P`: Perfect circle curve
    ///
    /// This function will return [Err(Error::InvalidData)][Error::InvalidData] if the string is
    /// not one of the following.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ty, points) = s
            .split_once('|')
            .ok_or(Error::InvalidData("invalid curve format"))?;

        let points = points
            .split('|')
            .map(str::trim)
            .map(|s| {
                s.split_once(':')
                    .ok_or(Error::InvalidData("invalid point format"))
            })
            .and_then(|(x, y)| {
                let x = x.parse().map_err(Error::from)?;
                let y = y.parse().map_err(Error::from)?;
                Ok(Vec2::new(x, y))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(match ty {
            "B" => Curve::Bezier(points),
            "C" => Curve::CatmullRom(points),
            "L" => Curve::Linear(points),
            "P" => Curve::Perfect(points),
            _ => return Err(Error::InvalidData("invalid curve type")),
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slider {
    /// Anchor points used to construct the slider.
    pub curve: Curve,
    /// Amount of times the player has to follow the slider's curve back-and-forth before the slider is complete. It can also be interpreted as the repeat count plus one.
    pub slides: u32,
    /// Visual length in osu! pixels of the slider.
    pub length: f32,
    /// Hitsounds that play when hitting edges of the slider's curve.
    /// The first sound is the one that plays when the slider is first clicked, and the last sound is the one that plays when the slider's end is hit.
    pub edge_sounds: Vec<HitSound>,
    /// Sample sets used for the edgeSounds.
    pub edge_sets: Vec<(SampleSet, SampleSet)>,
}

impl Slider {
    pub fn from_parts<'a>(mut parts: impl Iterator<Item = &'a str>) -> Result<Self, Error> {
        let curve = parts
            .next()
            .ok_or(Error::InvalidData("expected curve while parsing slider"))
            .and_then(Curve::from_str)?;

        let slides = parts
            .next()
            .ok_or(Error::InvalidData("expected slides while parsing slider"))
            .and_then(|s| s.parse().map_err(Error::from))?;

        let length = parts
            .next()
            .ok_or(Error::InvalidData("expected length while parsing slider"))
            .and_then(|s| s.parse().map_err(Error::from))?;

        let edge_sounds = if let Some(parts) = parts.next() {
            Some(
                parts
                    .split('|')
                    .map(u8::from_str)
                    .ok_map(HitSound::from_bits_truncate)
                    .collect::<Result<Vec<_>, _>>()?,
            )
        } else {
            None
        };

        let edge_sounds = edge_sounds.unwrap_or_default();

        let edge_sets = if let Some(parts) = parts.next().map(|s| s.split('|')) {
            Some(
                parts
                    .map(|s| {
                        s.split_once(':')
                            .ok_or(Error::InvalidData("invalid edge set format"))
                    })
                    .and_then(|(normal, addition)| {
                        Ok((i32::from_str(normal)?, i32::from_str(addition)?))
                    })
                    .and_then(|(normal, addition)| {
                        Ok((SampleSet::try_from(normal)?, SampleSet::try_from(addition)?))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )
        } else {
            None
        };

        let edge_sets = edge_sets.unwrap_or_default();

        Ok(Slider {
            curve,
            slides,
            length,
            edge_sounds,
            edge_sets,
        })
    }
}
