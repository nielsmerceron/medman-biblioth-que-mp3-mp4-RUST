use crate::allfile::{Allfile, MusicFile, VideoFile};
use crate::fonctminim::{getinfoa, getinfob};
use crate::scrap::scrap;
use crate::tag::{tagchangem, tagchangev};
use std::path::Path;

/// la fonction help est une fonction qu'on appelle pour savoir ce que fait une commande
pub fn help(inter: &str) {
    if inter.contains("scan") {
        println! {"  la fonction scan est une fonction scannant un répertoir. \n Pour utiliser cette fonction taper scan \n  puis la fonction vous demandera un path qui devra être de la forme /blabal/blabla/nom_du_répertoir a scan \n    ou d'utiliser un .json. si vous utilisez le json, indiqué le nom du fichier que vous aurez mis préalablement dans le dossier du programme\n"};
    }

    if inter.contains("search") {
        println! {"  la fonction search est une fonction cherchant dans un répertoir. \n Pour utiliser cette fonction taper search \n  puis la fonction vous demandera l'interogation d'un champ.\n  les champs interrogeables pour les mp3 sont : \n -artist \n -title \n -album \n -genre \n -date \n  les champs interrogeables pour les mp4 sont : \n -artist \n -title \n -genre \n -date \n    puis la fonction vous proposera une recherche appronfondi \n "};
    }

    if inter.contains("switch") {
        println! {"  la fonction switch permet de passer de la bibliothèque mp3 a mp4 \n   exemple : switch mp4 \n "}
    }

    if inter.contains("playlist") {
        println! {"  fonction fonctionnant de la même manière que search, créant une playlist jouable pas vlc \n    pour utiliser cette fonction taper playlist et suivez les instructions\n"}
    }

    if inter.contains("scrap") {
        println!("  la fonction scrap permet de rechercher une musique sur internet via un nom d'artiste et un titre de musique. \n     si la musique est présente dans le répertoire il y ira changer les tags si ceci vous conviennent\n");
    }

    if inter.contains("tag") {
        println!("  la fonction tag permet de changer un tag d'une vidéo ou d'une musique \n    pour utiliser cette fonction taper tag et suivez les instructions\n")
    }
}
///fonction final simulant l'intéraction d'une bibliothèque de média
pub fn assis() {
    let mut vecm: Vec<MusicFile> = Vec::new();
    let mut vecv: Vec<VideoFile> = Vec::new();
    let mut change: String = "mp3".to_string();
    loop {
        println!("                                  vous utilisez la bibliothèque pour {:#?} \n voici les commandes possible: \n 0-help (+nom de commande) \n 1-scan (mp3/mp4) \n 2-search(mp3/mp4) \n 3-playlist(mp3/mp4) \n 4-scrap(mp3) \n 5-tag(mp3/mp4) \n 6-switch \n si vous voulez finir l'utilisation taper quit après la fin d'une requet \n ",change);
        println!("que voulez vous faire ?\n");
        let line = getinfoa();

        if line.contains("quit") {
            println!("fin du programme, a bientôt\n");
            break;
        } else if line.contains("help") {
            help(&line);
        } else if line.contains("switch") {
            if line.contains("mp3") {
                change = "mp3".to_string()
            }
            if line.contains("mp4") {
                change = "mp4".to_string()
            }
        } else if change == "mp3" {
            if line == *"scan" {
                println!("voulez vous utiliser un fichier enregistrer en .json? [y/n]\n");
                if getinfob() == *"y" {
                    vecm = MusicFile::deseria();
                } else {
                    vecm = MusicFile::scanall();
                    println!("voulez vous sauvegarder le scan dan un fichier json [y/n]? \n");
                    if getinfob() == *"y" {
                        MusicFile::seria(&vecm);
                    }
                }
            } else if line == *"search" {
                let requet = MusicFile::searchall(&vecm);
                MusicFile::write2md(&requet, "search".to_string());
            } else if line == *"playlist" {
                MusicFile::playlist(&vecm);
            } else if line == *"scrap" {
                scrap(&vecm).unwrap();
            } else if line == *"tag" {
                println!("voulez vous que je vous affiche toutes les musiques pour que vous puissiez avoir du choix? [y/n]");
                if getinfoa() == *"y" {
                    MusicFile::aff(&vecm);
                }
                println!("indiquer le path, l'artiste, l'album, le genre et la date\n");
                tagchangem(
                    Path::new(&getinfoa()),
                    &getinfoa(),
                    &getinfoa(),
                    &getinfoa(),
                    &getinfoa(),
                    &getinfoa(),
                );
            } else {
                println!("commande non reconnu\n");
            }
        } else if change == *"mp4" {
            if line == *"scan" {
                println!("voulez vous utiliser un fichier enregistrer en .json? [y/n]\n");
                if getinfob() == *"y" {
                    vecv = VideoFile::deseria();
                } else {
                    vecv = VideoFile::scanall();
                    println!("voulez vous sauvegarder le scan dan un fichier json [y/n]? \n");
                    if getinfob() == *"y" {
                        VideoFile::seria(&vecv);
                    }
                }
            } else if line == *"search" {
                let requet = VideoFile::searchall(&vecv);
                VideoFile::write2md(&requet, "search".to_string());
            } else if line == *"playlist" {
                VideoFile::playlist(&vecv);
            } else if line == *"tag" {
                println!("voulez vous que je vous affiche toutes les videos pour que vous puissiez avoir du choix? [y/n]");
                if getinfoa() == *"y" {
                    VideoFile::aff(&vecv);
                }
                println!("indiquer le path, l'artiste, le titre, le genre et la date\n");
                tagchangev(
                    Path::new(&getinfoa()),
                    &getinfoa(),
                    &getinfoa(),
                    &getinfoa(),
                    &getinfoa(),
                );
            } else {
                println!("commande non reconnu\n");
            }
        }
    }
}
