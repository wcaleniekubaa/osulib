use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
    str::FromStr,
};

use bitflags::bitflags;
use colour::Colours;
use difficulty::Difficulty;
use editor::Editor;
use event::EventKind;
use general::General;
use hit_objects::HitObject;
use metadata::Metadata;
use timing_point::TimingPoint;

use crate::Error;

pub mod colour;
pub mod difficulty;
pub mod editor;
pub mod event;
pub mod general;
pub mod hit_objects;
pub mod hit_sample;
pub mod hit_sound;
pub mod metadata;
pub mod sample_set;
pub mod timing_point;

pub trait Visitor {
    #[inline]
    fn open(path: impl AsRef<Path>, decoder: BeatmapDecoder) -> Result<Self, Error>
    where
        Self: Default,
    {
        decoder.parse(Self::default(), File::open(path)?)
    }

    #[inline]
    fn visit_file_format_version(&mut self, version: u32) {
        let _ = version;
    }

    #[inline]
    fn visit_general(&mut self, general: General) {
        let _ = general;
    }

    #[inline]
    fn visit_editor(&mut self, editor: Editor) {
        let _ = editor;
    }

    #[inline]
    fn visit_metadata(&mut self, metadata: Metadata) {
        let _ = metadata;
    }

    #[inline]
    fn visit_difficulty(&mut self, difficulty: Difficulty) {
        let _ = difficulty;
    }

    #[inline]
    fn visit_events(&mut self, events: impl Iterator<Item = EventKind>) {
        let _ = events;
    }

    #[inline]
    fn visit_timing_points(&mut self, timing_points: impl Iterator<Item = TimingPoint>) {
        let _ = timing_points;
    }

    #[inline]
    fn visit_colours(&mut self, colours: Colours) {
        let _ = colours;
    }

    #[inline]
    fn visit_hit_objects(&mut self, hit_objects: impl Iterator<Item = HitObject>) {
        let _ = hit_objects;
    }
}

impl<T: Visitor> Visitor for &mut T {
    #[inline]
    fn visit_file_format_version(&mut self, version: u32) {
        T::visit_file_format_version(self, version)
    }

    #[inline]
    fn visit_general(&mut self, general: General) {
        T::visit_general(self, general)
    }

    #[inline]
    fn visit_editor(&mut self, editor: Editor) {
        T::visit_editor(self, editor)
    }

    #[inline]
    fn visit_metadata(&mut self, metadata: Metadata) {
        T::visit_metadata(self, metadata)
    }

    #[inline]
    fn visit_difficulty(&mut self, difficulty: Difficulty) {
        T::visit_difficulty(self, difficulty)
    }

    #[inline]
    fn visit_events(&mut self, events: impl Iterator<Item = EventKind>) {
        T::visit_events(self, events)
    }

    #[inline]
    fn visit_timing_points(&mut self, timing_points: impl Iterator<Item = TimingPoint>) {
        T::visit_timing_points(self, timing_points)
    }

    #[inline]
    fn visit_colours(&mut self, colours: Colours) {
        T::visit_colours(self, colours)
    }

    #[inline]
    fn visit_hit_objects(&mut self, hit_objects: impl Iterator<Item = HitObject>) {
        T::visit_hit_objects(self, hit_objects)
    }
}

impl<T: Visitor> Visitor for Box<T> {
    #[inline]
    fn visit_file_format_version(&mut self, version: u32) {
        T::visit_file_format_version(self, version)
    }

    #[inline]
    fn visit_general(&mut self, general: General) {
        T::visit_general(self, general)
    }

    #[inline]
    fn visit_editor(&mut self, editor: Editor) {
        T::visit_editor(self, editor)
    }

    #[inline]
    fn visit_metadata(&mut self, metadata: Metadata) {
        T::visit_metadata(self, metadata)
    }

    #[inline]
    fn visit_difficulty(&mut self, difficulty: Difficulty) {
        T::visit_difficulty(self, difficulty)
    }

    #[inline]
    fn visit_events(&mut self, events: impl Iterator<Item = EventKind>) {
        T::visit_events(self, events)
    }

    #[inline]
    fn visit_timing_points(&mut self, timing_points: impl Iterator<Item = TimingPoint>) {
        T::visit_timing_points(self, timing_points)
    }

    #[inline]
    fn visit_colours(&mut self, colours: Colours) {
        T::visit_colours(self, colours)
    }

    #[inline]
    fn visit_hit_objects(&mut self, hit_objects: impl Iterator<Item = HitObject>) {
        T::visit_hit_objects(self, hit_objects)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Beatmap {
    pub file_format_version: u8,
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Vec<EventKind>,
    pub timing_points: Vec<TimingPoint>,
    pub colours: Colours,
    pub hit_objects: Vec<HitObject>,
}

impl Visitor for Beatmap {
    #[inline]
    fn visit_file_format_version(&mut self, version: u32) {
        self.file_format_version = version as u8;
    }

    #[inline]
    fn visit_general(&mut self, general: General) {
        self.general = general;
    }

    #[inline]
    fn visit_editor(&mut self, editor: Editor) {
        self.editor = editor;
    }

    #[inline]
    fn visit_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    #[inline]
    fn visit_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    #[inline]
    fn visit_events(&mut self, events: impl Iterator<Item = EventKind>) {
        self.events = events.collect();
    }

    #[inline]
    fn visit_timing_points(&mut self, timing_points: impl Iterator<Item = TimingPoint>) {
        self.timing_points = timing_points.collect();
    }

    #[inline]
    fn visit_colours(&mut self, colours: Colours) {
        self.colours = colours;
    }

    #[inline]
    fn visit_hit_objects(&mut self, hit_objects: impl Iterator<Item = HitObject>) {
        self.hit_objects = hit_objects.collect();
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct BeatmapDecoder: u32 {
        const GENERAL = 1 << 0;
        const EDITOR = 1 << 1;
        const METADATA = 1 << 2;
        const DIFFICULTY = 1 << 3;
        const EVENTS = 1 << 4;
        const TIMING_POINTS = 1 << 5;
        const COLOURS = 1 << 6;
        const HIT_OBJECTS = 1 << 7;
    }
}

impl BeatmapDecoder {
    #[inline]
    pub const fn minimal() -> Self {
        Self::empty()
            .general()
            .metadata()
            .difficulty()
            .events()
            .timing_points()
    }

    #[inline]
    pub const fn general(self) -> Self {
        self.union(BeatmapDecoder::GENERAL)
    }

    #[inline]
    pub const fn editor(self) -> Self {
        self.union(BeatmapDecoder::EDITOR)
    }

    #[inline]
    pub const fn metadata(self) -> Self {
        self.union(BeatmapDecoder::METADATA)
    }

    #[inline]
    pub const fn difficulty(self) -> Self {
        self.union(BeatmapDecoder::DIFFICULTY)
    }

    #[inline]
    pub const fn events(self) -> Self {
        self.union(BeatmapDecoder::EVENTS)
    }

    #[inline]
    pub const fn timing_points(self) -> Self {
        self.union(BeatmapDecoder::TIMING_POINTS)
    }

    #[inline]
    pub const fn colours(self) -> Self {
        self.union(BeatmapDecoder::COLOURS)
    }

    #[inline]
    pub const fn hit_objects(self) -> Self {
        self.union(BeatmapDecoder::HIT_OBJECTS)
    }

    pub fn parse<Vis>(self, mut beatmap: Vis, data: impl Read) -> Result<Vis, Error>
    where
        Vis: Visitor,
    {
        let data = io::BufReader::new(data);

        let mut section = String::new();

        let mut general = if self.contains(BeatmapDecoder::GENERAL) {
            Some(General::default())
        } else {
            None
        };
        let mut editor = if self.contains(BeatmapDecoder::EDITOR) {
            Some(Editor::default())
        } else {
            None
        };
        let mut metadata = if self.contains(BeatmapDecoder::METADATA) {
            Some(Metadata::default())
        } else {
            None
        };
        let mut difficulty = if self.contains(BeatmapDecoder::DIFFICULTY) {
            Some(Difficulty::default())
        } else {
            None
        };

        let mut events = if self.contains(BeatmapDecoder::EVENTS) {
            Some(Vec::with_capacity(8))
        } else {
            None
        };
        let mut timing_points = if self.contains(BeatmapDecoder::TIMING_POINTS) {
            Some(Vec::with_capacity(128))
        } else {
            None
        };
        let mut colours = if self.contains(BeatmapDecoder::COLOURS) {
            Some(Colours::default())
        } else {
            None
        };
        let mut hit_objects = if self.contains(BeatmapDecoder::HIT_OBJECTS) {
            Some(Vec::with_capacity(1024))
        } else {
            None
        };

        for line in data.lines() {
            let line = line?;

            if line.starts_with("osu file format v") {
                let version = line[16..].trim().parse::<u32>()?;
                beatmap.visit_file_format_version(version);
                continue;
            }

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("[") && line.ends_with("]") {
                section = line[1..line.len() - 1].to_string();
                continue;
            }

            match section.as_str() {
                "General" => {
                    if let Some(general) = general.as_mut() {
                        general.parse(&line)?;
                    }
                }
                "Editor" => {
                    if let Some(editor) = editor.as_mut() {
                        editor.parse(&line)?;
                    }
                }
                "Metadata" => {
                    if let Some(metadata) = metadata.as_mut() {
                        metadata.parse(&line)?;
                    }
                }
                "Difficulty" => {
                    if let Some(difficulty) = difficulty.as_mut() {
                        difficulty.parse(&line)?;
                    }
                }
                "TimingPoints" => {
                    if let Some(timing_points) = timing_points.as_mut() {
                        timing_points.push(TimingPoint::from_str(&line)?);
                    }
                }
                "Events" => {
                    if let Some(events) = events.as_mut() {
                        if let Ok(event) = EventKind::from_str(&line) {
                            events.push(event);
                        }
                    }
                }
                "Colours" => {
                    if let Some(colours) = colours.as_mut() {
                        colours.parse(&line)?;
                    }
                }
                "HitObjects" => {
                    if let Some(hit_objects) = hit_objects.as_mut() {
                        hit_objects.push(HitObject::from_str(&line)?);
                    }
                }
                _ => (),
            }
        }

        if let Some(general) = general {
            beatmap.visit_general(general);
        }
        if let Some(editor) = editor {
            beatmap.visit_editor(editor);
        }
        if let Some(metadata) = metadata {
            beatmap.visit_metadata(metadata);
        }
        if let Some(difficulty) = difficulty {
            beatmap.visit_difficulty(difficulty);
        }
        if let Some(events) = events {
            beatmap.visit_events(events.into_iter());
        }
        if let Some(timing_points) = timing_points {
            beatmap.visit_timing_points(timing_points.into_iter());
        }
        if let Some(colours) = colours {
            beatmap.visit_colours(colours);
        }
        if let Some(hit_objects) = hit_objects {
            beatmap.visit_hit_objects(hit_objects.into_iter());
        }

        Ok(beatmap)
    }
}

impl Default for BeatmapDecoder {
    #[inline]
    fn default() -> Self {
        Self::all()
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MinimalBeatmap {
    pub general: General,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Vec<EventKind>,
    pub timing_points: Vec<TimingPoint>,
}

impl Visitor for MinimalBeatmap {
    #[inline]
    fn visit_general(&mut self, general: General) {
        self.general = general;
    }

    #[inline]
    fn visit_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    #[inline]
    fn visit_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    #[inline]
    fn visit_timing_points(&mut self, timing_points: impl Iterator<Item = TimingPoint>) {
        self.timing_points = timing_points.collect();
    }

    #[inline]
    fn visit_events(&mut self, events: impl Iterator<Item = EventKind>) {
        self.events = events.collect();
    }
}
