use crate::Error;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Difficulty {
    /// HP setting (0–10)
    pub hp: f32,
    /// CS setting (0–10)
    pub cs: f32,
    /// OD setting (0–10)
    pub od: f32,
    /// AR setting (0–10)
    pub ar: f32,
    /// Base slider velocity in hundreds of osu! pixels per beat
    pub slider_multiplier: f32,
    /// Amount of slider ticks per beat
    pub slider_tick_rate: f32,
}

impl Default for Difficulty {
    #[inline]
    fn default() -> Self {
        Self {
            hp: 5.0,
            cs: 5.0,
            od: 5.0,
            ar: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
        }
    }
}

impl Difficulty {
    pub fn parse(&mut self, s: &str) -> Result<(), Error> {
        if let Some((key, value)) = s.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "HPDrainRate" => self.hp = value.parse()?,
                "CircleSize" => self.cs = value.parse()?,
                "OverallDifficulty" => self.od = value.parse()?,
                "ApproachRate" => self.ar = value.parse()?,
                "SliderMultiplier" => self.slider_multiplier = value.parse()?,
                "SliderTickRate" => self.slider_tick_rate = value.parse()?,
                _ => {}
            }
        }

        Ok(())
    }
}
