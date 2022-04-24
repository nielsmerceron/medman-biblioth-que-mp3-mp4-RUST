use crate::scan::{scanm, scanv};
use crate::search::{searchvecm, searchvecv};
use crate::seria::{deseriam, deseriav, serialvecm, serialvecv};
use crate::write::{playlistm, playlistv};
use crate::write2m::{write2mm, write2mv};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Serialize, Debug, Deserialize)]
pub struct MusicFile {
    pub path: PathBuf,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub genre: String,
    pub date: Option<i32>,
}
impl MusicFile {
    pub fn newm(
        path: &Path,
        artist: &str,
        title: &str,
        album: &str,
        kind: &str,
        date: Option<i32>,
    ) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            artist: artist.to_string(),
            title: title.to_string(),
            album: album.to_string(),
            genre: kind.to_string(),
            date,
        }
    }

    pub fn affm(&self) {
        println!("{:#?}", self.path);
        println!("{:#?}", self.artist);
        println!("{:#?}", self.title);
        println!("{:#?}", self.album);
        println!("{:#?}", self.genre);
        println!("{:#?}", self.date);
        println!("\n");
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct VideoFile {
    pub path: PathBuf,
    pub artist: String,
    pub title: String,
    pub duration: Duration,
    pub genre: String,
    pub date: String,
}

impl VideoFile {
    pub fn newv(
        path: &Path,
        artist: &str,
        title: &str,
        duration: Duration,
        kind: &str,
        date: &str,
    ) -> VideoFile {
        VideoFile {
            path: path.to_path_buf(),
            artist: artist.to_string(),
            title: title.to_string(),
            duration,
            genre: kind.to_string(),
            date: date.to_string(),
        }
    }
    pub fn affv(&self) {
        println!("{:#?}", self.path);
        println!("{:#?}", self.artist);
        println!("{:#?}", self.title);
        println!("{:#?}", self.duration);
        println!("{:#?}", self.genre);
        println!("{:#?}", self.date);
        println!("\n");
    }
}

pub trait Allfile {
    type Instance;
    fn scanall() -> Vec<Self::Instance>;
    fn searchall(vecteur: &Vec<Self::Instance>) -> Vec<Self::Instance>;
    fn seria(vecteur: &Vec<Self::Instance>);
    fn deseria() -> Vec<Self::Instance>;
    fn write2md(vecteur: &Vec<Self::Instance>, requet: String);
    fn playlist(vecteur: &Vec<Self::Instance>);
    fn aff(vecteur: &Vec<Self::Instance>);
}

impl Allfile for MusicFile {
    type Instance = MusicFile;
    fn scanall() -> Vec<Self::Instance> {
        scanm()
    }
    fn searchall(vecteur: &Vec<Self::Instance>) -> Vec<Self::Instance> {
        searchvecm(vecteur)
    }
    fn seria(vecteur: &Vec<Self::Instance>) {
        serialvecm(vecteur);
    }
    fn deseria() -> Vec<Self::Instance> {
        deseriam()
    }
    fn write2md(vecteur: &Vec<Self::Instance>, requet: String) {
        write2mm(vecteur, requet);
    }
    fn playlist(vecteur: &Vec<Self::Instance>) {
        playlistm(vecteur);
    }
    fn aff(vecteur: &Vec<Self::Instance>) {
        for i in vecteur {
            i.affm();
        }
    }
}

impl Allfile for VideoFile {
    type Instance = VideoFile;
    fn scanall() -> Vec<Self::Instance> {
        scanv()
    }
    fn searchall(vecteur: &Vec<Self::Instance>) -> Vec<Self::Instance> {
        searchvecv(vecteur)
    }
    fn seria(vecteur: &Vec<Self::Instance>) {
        serialvecv(vecteur);
    }
    fn deseria() -> Vec<Self::Instance> {
        deseriav()
    }
    fn write2md(vecteur: &Vec<Self::Instance>, requet: String) {
        write2mv(vecteur, requet);
    }
    fn playlist(vecteur: &Vec<Self::Instance>) {
        playlistv(vecteur);
    }
    fn aff(vecteur: &Vec<Self::Instance>) {
        for i in vecteur {
            i.affv();
        }
    }
}
