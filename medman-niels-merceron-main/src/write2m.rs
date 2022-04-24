use crate::allfile::{MusicFile, VideoFile};
use crate::fonctminim::{convertoptioni32toi32, convertpathbuftostring, getinfob};
use markdown_gen::markdown::{AsMarkdown, List, Markdown};
use std::fs::File;

/// la fonction write2mM prend en argument &Vec<MUsicfile> et un String puis l'écrit dans un fichier markdow
pub fn write2mm(vecteur: &Vec<MusicFile>, requet: String) {
    println!("donnez un nom au fichier.md:\n");
    let file = File::create(getinfob() + ".md").expect("mauvaise création du fichier .md");
    let mut md = Markdown::new(file);
    md.write(requet.heading(1))
        .expect("ne peux pas écrire dans le .md");
    for i in vecteur {
        md.write(
            List::new(true)
                .title(&convertpathbuftostring(&i.path)[..])
                .item(&i.title[..])
                .item(&i.artist[..])
                .item(&i.album[..])
                .item(&i.genre[..])
                .item(&convertoptioni32toi32(i.date)[..]),
        )
        .expect("ne peux pas écrire dans le .md");
        md.write("").unwrap();
    }
}

/// la fonction write2mV prend en argument &Vec<VideoFile> et un String puis l'écrit dans un fichier markdow
pub fn write2mv(vecteur: &Vec<VideoFile>, requet: String) {
    println!("donnez un nom au fichier.md:\n");
    let file = File::create(getinfob() + ".md").expect("mauvaise création du fichier .md");
    let mut md = Markdown::new(file);
    md.write(requet.heading(1))
        .expect("ne peux pas écrire dans le .md");
    for i in vecteur {
        md.write(
            List::new(true)
                .title(&convertpathbuftostring(&i.path)[..])
                .item(&i.title[..])
                .item(&i.artist[..])
                .item(&*i.duration.as_secs_f32().to_string())
                .item(&i.genre[..])
                .item(&i.date[..]),
        )
        .expect("ne peux pas écrire dans le .md");
        md.write("").unwrap();
    }
}
///fonction global reproduisant une intéraction pour le test
pub fn write2mtest(vecteur: &Vec<MusicFile>, requet: String) {
    println!("donnez un nom au fichier.md:\n");
    let file = File::create("test".to_string() + ".md").expect("mauvaise création du fichier .md");
    let mut md = Markdown::new(file);
    md.write(requet.heading(1))
        .expect("ne peux pas écrire dans le .md");
    for i in vecteur {
        md.write(
            List::new(true)
                .title(&convertpathbuftostring(&i.path)[..])
                .item(&i.title[..])
                .item(&i.artist[..])
                .item(&i.album[..])
                .item(&i.genre[..])
                .item(&convertoptioni32toi32(i.date)[..]),
        )
        .expect("ne peux pas écrire dans le .md");
        md.write("").unwrap();
    }
}
