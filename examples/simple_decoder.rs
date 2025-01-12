use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use osulib::file::beatmap::{Beatmap, BeatmapDecoder, Visitor};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path = &args[1];
    let path = PathBuf::from(path);

    let beatmap = Beatmap::open(&path, BeatmapDecoder::all()).expect("Beatmap::open failed");

    let output = PathBuf::from(format!(
        "{}.txt",
        path.file_stem().expect("file_stem").to_string_lossy()
    ));

    fs::write(output, format!("{:#?}", beatmap)).expect("fs::write failed");
}
