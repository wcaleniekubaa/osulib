pub mod countdown;
pub mod overlay_position;

use std::{path::PathBuf, str::FromStr};

use countdown::Countdown;
use overlay_position::OverlayPosition;

use crate::{mode::Mode, Error};

use super::sample_set::SampleSet;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct General {
    /// Location of the audio file relative to the current folder
    pub audio_filename: PathBuf,
    /// Milliseconds of silence before the audio starts playing
    pub audio_lead_in: i32,
    /// Time in milliseconds when the audio preview should start
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub preview_time: Option<i32>,
    /// Speed of the countdown before the first hit object
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub countdown: Option<Countdown>,
    /// Sample set that will be used if timing points do not override it
    pub sample_set: SampleSet,
    /// Multiplier for the threshold in time where hit objects placed close together stack (0–1)
    pub stack_leniency: f32,
    /// Game mode
    pub mode: Mode,
    /// Whether or not breaks have a letterboxing effect
    pub letterbox_in_breaks: bool,
    /// Whether or not the storyboard can use the user's skin images
    pub use_skin_sprites: bool,
    /// Draw order of hit circle overlays compared to hit numbers
    /// - [`None`] = use skin setting,
    /// - [`Some(Below)`][OverlayPosition::Below] = draw overlays below numbers,
    /// - [`Some(Above)`][OverlayPosition::Above] = draw overlays on top of numbers
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub overlay_position: Option<OverlayPosition>,
    /// Preferred skin to use during gameplay
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub skin_preference: Option<String>,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map
    pub epilepsy_warning: bool,
    /// Time in beats that the countdown starts before the first hit object
    pub countdown_offset: i32,
    /// Whether or not the "N+1" style key layout is used for osu!mania
    pub special_style: bool,
    /// Whether or not the storyboard allows widescreen viewing
    pub widescreen_storyboard: bool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods
    pub samples_match_playback_rate: bool,
}

impl Default for General {
    #[inline]
    fn default() -> Self {
        Self {
            audio_filename: PathBuf::new(),
            audio_lead_in: 0,
            preview_time: None,
            countdown: None,
            sample_set: SampleSet::default(),
            stack_leniency: 0.7,
            mode: Mode::Standard,
            letterbox_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: None,
            skin_preference: None,
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: true,
        }
    }
}

impl General {
    pub fn parse(&mut self, s: &str) -> Result<(), Error> {
        if let Some((key, value)) = s.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "AudioFilename" => self.audio_filename = value.into(),
                "AudioLeadIn" => self.audio_lead_in = value.parse()?,
                "PreviewTime" => self.preview_time = Some(value.parse()?),
                "Countdown" => self.countdown = Some(Countdown::try_from(value.parse::<i32>()?)?),
                "SampleSet" => self.sample_set = SampleSet::from_str(value)?,
                "StackLeniency" => self.stack_leniency = value.parse()?,
                "Mode" => self.mode = Mode::try_from(value.parse::<i32>()?)?,
                "LetterboxInBreaks" => self.letterbox_in_breaks = value.parse::<u8>()? != 0,
                "UseSkinSprites" => self.use_skin_sprites = value.parse::<u8>()? != 0,
                "OverlayPosition" => {
                    self.overlay_position = match value {
                        "NoChange" => None,
                        _ => Some(OverlayPosition::from_str(value)?),
                    }
                }
                "SkinPreference" => self.skin_preference = Some(value.into()),
                "EpilepsyWarning" => self.epilepsy_warning = value.parse::<u8>()? != 0,
                "CountdownOffset" => self.countdown_offset = value.parse()?,
                "SpecialStyle" => self.special_style = value.parse::<u8>()? != 0,
                "WidescreenStoryboard" => self.widescreen_storyboard = value.parse::<u8>()? != 0,
                "SamplesMatchPlaybackRate" => {
                    self.samples_match_playback_rate = value.parse::<u8>()? != 0
                }

                _ => {}
            }
        }
        Ok(())
    }
}
