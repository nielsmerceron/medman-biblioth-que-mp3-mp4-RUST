use crate::fonctminim::convertstringtoi32;
use id3::{Tag, Version};
use mp4ameta::Tag as Tagmp4;
use std::path::Path;

///fontion changeant le tag d'une musique
pub fn tagchangem(path: &Path, artist: &str, title: &str, album: &str, genre: &str, date: &str) {
    if let Ok(mut tag) = Tag::read_from_path(path) {
        tag.set_artist(artist);
        tag.set_title(title);
        tag.set_album(album);
        tag.set_genre(genre);
        tag.set_year(convertstringtoi32(&date.to_string()));

        tag.write_to_path(path, Version::Id3v22)
            .expect("erreur lors du changement du tag");
    } else {
        println!("le path ne permet pas d'accéder au fichier que vous voulez modifier\n")
    }
}

/// fonction changeant le atg d'une vidéo
pub fn tagchangev(path: &Path, artist: &str, title: &str, genre: &str, date: &str) {
    if let Ok(mut tag) = Tagmp4::read_from_path(path) {
        tag.set_artist(artist);
        tag.set_title(title);
        tag.set_genre(genre);
        tag.set_year(date);

        tag.write_to_path(path)
            .expect("erreur lors du changement du tag");
    } else {
        println!("le path ne permet pas d'accéder au fichier que vous voulez modifier\n")
    }
}
