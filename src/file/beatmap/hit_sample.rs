use std::str::FromStr;

use crate::Error;

use super::sample_set::SampleSet;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HitSample {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub normal_set: Option<SampleSet>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub addition_set: Option<SampleSet>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub index: Option<i32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub volume: Option<i32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub filename: Option<String>,
}

impl FromStr for HitSample {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let normal_set = parts.next().ok_or(Error::InvalidData(
            "expected normal set while parsing hit sample",
        ))?;
        let addition_set = parts.next().ok_or(Error::InvalidData(
            "expected addition set while parsing hit sample",
        ))?;
        let index = parts.next().ok_or(Error::InvalidData(
            "expected index while parsing hit sample",
        ))?;
        let volume = parts.next().ok_or(Error::InvalidData(
            "expected volume while parsing hit sample",
        ))?;
        let filename = parts.next().ok_or(Error::InvalidData(
            "expected filename while parsing hit sample",
        ))?;

        let normal_set = if normal_set == "0" {
            None
        } else {
            Some(SampleSet::try_from(normal_set.parse::<i32>()?)?)
        };

        let addition_set = if addition_set == "0" {
            None
        } else {
            Some(SampleSet::try_from(addition_set.parse::<i32>()?)?)
        };

        let index = if index == "0" {
            None
        } else {
            Some(index.parse()?)
        };

        let volume = if volume == "0" {
            None
        } else {
            Some(volume.parse()?)
        };

        let filename = if filename == "0" {
            None
        } else {
            Some(filename.to_string())
        };

        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}
