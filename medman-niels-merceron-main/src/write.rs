use crate::allfile::{MusicFile, VideoFile};
use crate::fonctminim::getinfob;
use crate::search::{searchvecm, searchvecv};
use pls::*;
use std::fs::File;
use std::io::Write;

///playlistM créé a l'aide d'un &Vec<MusicFile>, un fichier pls pouvant être lu par vlc
pub fn playlistm(vecteur: &Vec<MusicFile>) {
    println!("nommé le champs que vous recherché (title,artist,album,date,genre)\n puis nommé ce que vous voulez \n");
    let requet = searchvecm(vecteur);
    println!("nommé le titre de la playlist\n");
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut file =
        File::create(getinfob() + ".pls").expect("mauvaise créatrion du fichier de playlist\n");
    for i in requet {
        buf.push(PlaylistElement {
            path: i
                .path
                .into_os_string()
                .into_string()
                .expect("mauvaise conversion du path\n"),
            title: Some(i.title),
            len: ElementLength::Unknown,
        })
    }
    pls::write(&buf[..], &mut buf2).expect("mauvaise écriture en pls");

    file.write_all(&buf2)
        .expect("erreur d'écriture du fichier.pls\n");
}

///playlistV créé a l'aide d'un &Vec<VideoFile>, un fichier pls pouvant être lu par vlc
pub fn playlistv(vecteur: &Vec<VideoFile>) {
    println!("nommé le champs que vous recherché (title,artist,album,date,genre)\n puis nommé ce que vous voulez \n");
    let requet = searchvecv(vecteur);
    println!("nommé le titre de la playlist\n");
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();
    let mut file =
        File::create(getinfob() + ".pls").expect("mauvaise créatrion du fichier de playlist\n");
    for i in requet {
        buf.push(PlaylistElement {
            path: i
                .path
                .into_os_string()
                .into_string()
                .expect("mauvaise conversion du path\n"),
            title: Some(i.title),
            len: ElementLength::Seconds(i.duration.as_secs_f64() as u64),
        })
    }
    pls::write(&buf[..], &mut buf2).expect("mauvaise écriture en pls");

    file.write_all(&buf2)
        .expect("erreur d'écriture du fichier.pls\n");
}
