use std::str::FromStr;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Colour(pub u8, pub u8, pub u8);

impl Colour {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb = s
            .split(',')
            .map(str::trim)
            .map(u8::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        if rgb.len() != 3 {
            return Err(Error::InvalidData(
                "expected three comma-separated numbers while parsing colour",
            ));
        }

        let [r, g, b] = rgb.as_slice() else {
            unreachable!()
        };

        Ok(Self(*r, *g, *b))
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Colours {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_combo_colours"))]
    pub combo_colours: [Option<Colour>; 8],
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub slider_track_override: Option<Colour>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub slider_border: Option<Colour>,
}

#[cfg(feature = "serde")]
fn serialize_combo_colours<S>(
    colours: &[Option<Colour>; 8],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;

    let combo_colours = colours
        .iter()
        .copied()
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    combo_colours.serialize(serializer)
}

impl Colours {
    #[inline]
    pub fn combo_colours(&self) -> impl Iterator<Item = (usize, Colour)> {
        self.combo_colours
            .into_iter()
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .enumerate()
    }

    pub fn parse(&mut self, s: &str) -> Result<(), Error> {
        if let Some((key, value)) = s.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "Combo1" => self.combo_colours[0] = Some(Colour::from_str(value)?),
                "Combo2" => self.combo_colours[1] = Some(Colour::from_str(value)?),
                "Combo3" => self.combo_colours[2] = Some(Colour::from_str(value)?),
                "Combo4" => self.combo_colours[3] = Some(Colour::from_str(value)?),
                "Combo5" => self.combo_colours[4] = Some(Colour::from_str(value)?),
                "Combo6" => self.combo_colours[5] = Some(Colour::from_str(value)?),
                "Combo7" => self.combo_colours[6] = Some(Colour::from_str(value)?),
                "Combo8" => self.combo_colours[7] = Some(Colour::from_str(value)?),
                "SliderBorder" => self.slider_border = Some(Colour::from_str(value)?),
                "SliderTrackOverride" => {
                    self.slider_track_override = Some(Colour::from_str(value)?)
                }
                _ => {}
            }
        }
        Ok(())
    }
}
