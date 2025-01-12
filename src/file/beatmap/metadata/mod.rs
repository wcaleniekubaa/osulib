use crate::Error;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata {
    ///  Romanised song title
    pub title: String,
    /// Song title
    pub title_unicode: String,
    /// Romanised song artist
    pub artist: String,
    /// Song artist
    pub artist_unicode: String,
    /// Beatmap creator
    pub creator: String,
    /// Difficulty name
    pub version: String,
    /// Original media the song was produced for
    pub source: String,
    /// Search terms
    pub tags: Vec<String>,
    /// Difficulty ID
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub beatmap_id: Option<i64>,
    /// Beatmap ID
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub beatmap_set_id: Option<i64>,
}

impl Metadata {
    pub fn parse(&mut self, s: &str) -> Result<(), Error> {
        if let Some((key, value)) = s.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "Title" => self.title = value.into(),
                "TitleUnicode" => self.title_unicode = value.into(),
                "Artist" => self.artist = value.into(),
                "ArtistUnicode" => self.artist_unicode = value.into(),
                "Creator" => self.creator = value.into(),
                "Version" => self.version = value.into(),
                "Source" => self.source = value.into(),
                "Tags" => {
                    self.tags = value
                        .split(' ')
                        .map(str::trim)
                        .map(str::to_string)
                        .collect()
                }
                "BeatmapID" => self.beatmap_id = Some(value.parse()?),
                "BeatmapSetID" => self.beatmap_set_id = Some(value.parse()?),

                _ => {}
            }
        }
        Ok(())
    }
}
