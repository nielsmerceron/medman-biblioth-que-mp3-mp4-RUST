use crate::allfile::{MusicFile, VideoFile};
use crate::fonctminim::{getinfoa, getinfob};

///searchvecM effectue une recherche (que ET géré) sur un &Vec<MusiFile> pour les catégories artist,title,album,genre,date
pub fn searchvecm(vecteur: &Vec<MusicFile>) -> Vec<MusicFile> {
    println!("choisissez le champs que vous voulez interroger:\n");
    let typee = getinfob();
    println!("tapez ce que vous cherchez:\n");
    let what = getinfoa();

    let mut compte: u32 = 0;
    let mut music_files: Vec<MusicFile> = Vec::new();
    println!("\n");
    for i in vecteur {
        if typee.contains("artist") && i.artist.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("title") && i.title.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("album") && i.album.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("kind") && i.genre.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("date") {
            match i.date {
                Some(w) => {
                    if w == what.parse::<i32>().unwrap() {
                        music_files.push(MusicFile::newm(
                            &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
                        ));
                        compte += 1;
                    }
                }
                None => {}
            }
        }
    }
    println!(
        "nb de music remplissant les conditions de recherches {:#?} \n",
        compte
    );
    println!("voulez vous faire une recherche plus profonde sur le résultat obtenu ? [y/n] \n");
    if getinfob().contains('y') {
        searchvecm(&music_files)
    } else {
        music_files
    }
}
///searchvecV effectue une recherche (que ET géré) sur un &Vec<VideoFile> pour les catégories artist,title,genre,date
pub fn searchvecv(vecteur: &Vec<VideoFile>) -> Vec<VideoFile> {
    println!("choisissez le champs que vous voulez interroger:\n");
    let typee = getinfob();
    println!("tapez ce que vous cherchez:\n");
    let what = getinfoa();

    let mut compte: u32 = 0;
    let mut video_file: Vec<VideoFile> = Vec::new();
    println!("\n");
    for i in vecteur {
        if typee.contains("artist") && i.artist.contains(&what) {
            //i.affv();
            video_file.push(VideoFile::newv(
                &i.path, &i.artist, &i.title, i.duration, &i.genre, &i.date,
            ));
            compte += 1;
        }
        if typee.contains("title") && i.title.contains(&what) {
            //i.affv();
            video_file.push(VideoFile::newv(
                &i.path, &i.artist, &i.title, i.duration, &i.genre, &i.date,
            ));
            compte += 1;
        }
        if typee.contains("kind") && i.genre.contains(&what) {
            //i.affv();
            video_file.push(VideoFile::newv(
                &i.path, &i.artist, &i.title, i.duration, &i.genre, &i.date,
            ));
            compte += 1;
        }
        if typee.contains("date") && i.date.contains(&what) {
            //i.affv();
            video_file.push(VideoFile::newv(
                &i.path, &i.artist, &i.title, i.duration, &i.genre, &i.date,
            ));
            compte += 1;
        }
    }
    println!(
        "nb de video remplissant les conditions de recherches {:#?} \n",
        compte
    );
    println!("voulez vous faire une recherche plus profonde sur le résultat obtenu ? [y/n] \n");
    if getinfob().contains('y') {
        searchvecv(&video_file)
    } else {
        video_file
    }
}

///fonction global pour les tests 
pub fn searchvectest(vecteur: &Vec<MusicFile>) -> Vec<MusicFile> {
    println!("choisissez le champs que vous voulez interroger:\n");
    let typee = "artist";
    println!("tapez ce que vous cherchez:\n");
    let what = "test";

    let mut compte: u32 = 0;
    let mut music_files: Vec<MusicFile> = Vec::new();
    println!("\n");
    for i in vecteur {
        if typee.contains("artist") && i.artist.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("title") && i.title.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("album") && i.album.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("kind") && i.genre.contains(&what) {
            //i.affm();
            music_files.push(MusicFile::newm(
                &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
            ));
            compte += 1;
        }
        if typee.contains("date") {
            match i.date {
                Some(w) => {
                    if w == what.parse::<i32>().unwrap() {
                        music_files.push(MusicFile::newm(
                            &i.path, &i.artist, &i.title, &i.album, &i.genre, i.date,
                        ));
                        compte += 1;
                    }
                }
                None => {}
            }
        }
    }
    println!(
        "nb de music remplissant les conditions de recherches {:#?} \n",
        compte
    );
    music_files

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonctminim::convertstringtooptioni32;
    use std::path::Path;

    #[test]
    fn testing_search_contient() {
        let mut vecteur: Vec<MusicFile> = Vec::new();
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"powerwolf".to_string(),
            &"sanctified with dynamite".to_string(),
            &"blood of the saints".to_string(),
            &"Metal".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"1".to_string(),
            &"1".to_string(),
            &"1".to_string(),
            &"Metal".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"test".to_string(),
            &"test".to_string(),
            &"test".to_string(),
            &"test".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));

        let requet=searchvectest(&vecteur);

        assert_eq!(requet[0].artist=="test",true);
    }

    #[test]
    fn testing_search_notcontenu() {
        let mut vecteur: Vec<MusicFile> = Vec::new();
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"powerwolf".to_string(),
            &"sanctified with dynamite".to_string(),
            &"blood of the saints".to_string(),
            &"Metal".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"1".to_string(),
            &"1".to_string(),
            &"1".to_string(),
            &"Metal".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));
        vecteur.push(MusicFile::newm(
            &Path::new("/home/niels/musique"),
            &"test".to_string(),
            &"test".to_string(),
            &"test".to_string(),
            &"test".to_string(),
            convertstringtooptioni32(&"2013".to_string()),
        ));

        let requet=searchvectest(&vecteur);

        assert_eq!(requet[0].artist=="1",false);
    }
}