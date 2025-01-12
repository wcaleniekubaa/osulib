use std::time::Duration;

use crate::file::beatmap::sample_set::SampleSet;

use super::Effects;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InheritedTimingPoint {
    /// Start time of the timing section, in milliseconds from the beginning of the beatmap's audio.
    /// The end of the timing section is the next timing point's time (or never, if this is the last timing point).
    pub time: Duration,
    /// Slider velocity multiplier.
    pub slider_velocity: f32,
    /// Default sample set for hit objects ([`None`] = beatmap default).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sample_set: Option<SampleSet>,
    /// Custom sample index for hit objects. [`None`] indicates osu!'s default hitsounds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sample_index: Option<i32>,
    /// Volume percentage for hit objects.
    pub volume: i32,
    /// Bit flags that give the timing point extra effects.
    pub effects: Effects,
}
