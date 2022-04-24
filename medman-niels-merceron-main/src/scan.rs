use crate::allfile::{MusicFile, VideoFile};
use crate::fonctminim::{convertoptost, getinfob};
use id3::Tag;
use mp4ameta::Tag as Tagmp4;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

///is_supportedmp3 est une fonction vérifiant si l'entré est au bon format(ex: mp3,txt,...)
const SUPPORTED_MP3: [&str; 1] = ["mp3"];
pub fn is_supportedmp3(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_MP3.contains(&entry.path().extension().unwrap().to_str().unwrap())
}
///scanm est une fonction demandant un path, collecte tous les audio en .mp3 et les mets dans une structure VideoFile puis dans un vect et renvoie ce vect
pub fn scanm() -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    println!("écrivez le path:");
    let walker = WalkDir::new(&Path::new(&getinfob())).into_iter();
    for entry in walker {
        let entry = entry.unwrap();
        if is_supportedmp3(&entry) {
            if let Ok(tag) = Tag::read_from_path(entry.path()) {
                let mf = MusicFile::newm(
                    entry.path(),
                    &convertoptost(tag.artist()),
                    &convertoptost(tag.title()),
                    &convertoptost(tag.album()),
                    &convertoptost(tag.genre()),
                    tag.year(),
                );
                music_files.push(mf);
            }
        }
    }
    music_files
}
///is_supportedmp4 est une fonction vérifiant si l'entré est au bon format(ex: mp4,mp3,txt,...)
const SUPPORTED_MP4: [&str; 1] = ["mp4"];
pub fn is_supportedmp4(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_MP4.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

///scanv est une fonction demandant un path, collecte tous les vidéos  en .mp4 et les mets dans une structure VideoFile puis dans un vect et renvoie ce vect
pub fn scanv() -> Vec<VideoFile> {
    let mut vid_files: Vec<VideoFile> = Vec::new();
    println!("écrivez le path:");
    let walker = WalkDir::new(&Path::new(&getinfob())).into_iter();
    for entry in walker {
        let entry = entry.expect("mauvaise entré");
        if is_supportedmp4(&entry) {
            if let Ok(tag) = Tagmp4::read_from_path(entry.path()) {
                let mf = VideoFile::newv(
                    entry.path(),
                    &convertoptost(tag.artist()),
                    &convertoptost(tag.title()),
                    tag.duration().unwrap(),
                    &convertoptost(tag.genre()),
                    &convertoptost(tag.year()),
                );
                vid_files.push(mf);
            }
        }
    }
    vid_files
}
