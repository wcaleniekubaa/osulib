use std::time::Duration;

use crate::{iter::OkMapExt, Error};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Editor {
    /// Time in milliseconds of bookmarks
    pub bookmarks: Vec<Duration>,
    /// Distance snap multiplier
    pub distance_spacing: f32,
    /// Beat snap divisor
    pub beat_divisor: i32,
    /// Grid size
    pub grid_size: i32,
    /// Scale factor for the object timeline
    pub timeline_zoom: f32,
}

impl Editor {
    pub fn parse(&mut self, s: &str) -> Result<(), Error> {
        if let Some((key, value)) = s.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "Bookmarks" => {
                    self.bookmarks = value
                        .split(',')
                        .map(str::trim)
                        .map(str::parse)
                        .ok_map(Duration::from_millis)
                        .collect::<Result<_, _>>()?;
                }
                "DistanceSpacing" => self.distance_spacing = value.parse()?,
                "BeatDivisor" => self.beat_divisor = value.parse()?,
                "GridSize" => self.grid_size = value.parse()?,
                "TimelineZoom" => self.timeline_zoom = value.parse()?,
                _ => {}
            }
        }
        Ok(())
    }
}
