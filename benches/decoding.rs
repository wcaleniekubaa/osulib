#![feature(test)]

extern crate test;

use std::io::Cursor;

use test::Bencher;

use osulib::file::beatmap::{Beatmap, BeatmapDecoder, MinimalBeatmap};

#[bench]
fn full_decode(b: &mut Bencher) {
    b.iter(|| {
        let _ = BeatmapDecoder::default().parse(
            Beatmap::default(),
            Cursor::new(&include_bytes!("./beatmap.osu")),
        );
    });
}

#[bench]
fn minimal_decode(b: &mut Bencher) {
    b.iter(|| {
        let _ = BeatmapDecoder::minimal().parse(
            MinimalBeatmap::default(),
            Cursor::new(&include_bytes!("./beatmap.osu")),
        );
    });
}
