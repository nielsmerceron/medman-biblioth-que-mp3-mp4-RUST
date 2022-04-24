use crate::allfile::{MusicFile, VideoFile};
use crate::fonctminim::getinfob;
use std::fs;

///serialvecM créé a l'aide d'un &Vec<MusicFile>, un fichier .json pour stocker les informations d'une bibliothèque scanné
pub fn serialvecm(vecteur: &Vec<MusicFile>) {
    let serialized: String = serde_json::to_string_pretty(vecteur).expect("Could not serialized\n");
    let mut strin: String =
        "/home/niels/github-classroom/uvsq-hal/medman-niels-merceron/".to_owned();
    println!("nommé votre fichier (respecter l'écriture snake) : \n");
    strin.push_str(&getinfob());
    strin.push_str(".json");
    fs::write(strin, serialized).expect("ça n'a pas écrit");
}

///sdeseriaM créé a l'aide d'un fichier .json un Vec<MusicFile>
pub fn deseriam() -> Vec<MusicFile> {
    println!("écrivez le nom du fichier (.json compris)\n");
    let vecteur: Vec<MusicFile> =
        serde_json::from_str(&fs::read_to_string(getinfob()).expect("erreur de lecture\n"))
            .expect("erreur de désérialisation\n");
    vecteur
}

///serialvecV créé a l'aide d'un &Vec<VideoFile>, un fichier .json pour stocker les informations d'une bibliothèque scanné
pub fn serialvecv(vecteur: &Vec<VideoFile>) {
    let serialized: String = serde_json::to_string_pretty(vecteur).expect("Could not serialized\n");
    let mut strin: String =
        "/home/niels/github-classroom/uvsq-hal/medman-niels-merceron/".to_owned();
    println!("nommé votre fichier (respecter l'écriture snake) : \n");
    strin.push_str(&getinfob());
    strin.push_str(".json");
    fs::write(strin, serialized).expect("ça n'a pas écrit");
}

///sdeseriaV créé a l'aide d'un fichier .json un Vec<VideoFile>
pub fn deseriav() -> Vec<VideoFile> {
    println!("écrivez le nom du fichier (.json compris)\n");
    let vecteur: Vec<VideoFile> =
        serde_json::from_str(&fs::read_to_string(getinfob()).expect("erreur de lecture\n"))
            .expect("erreur de désérialisation\n");
    vecteur
}

///fonction global reprodusant une intéraction pour les tests
pub fn serialvectest(vecteur: &Vec<MusicFile>) {
    let serialized: String = serde_json::to_string_pretty(vecteur).expect("Could not serialized\n");
    let mut strin: String =
        "/home/niels/github-classroom/uvsq-hal/medman-niels-merceron/".to_owned();
    println!("nommé votre fichier (respecter l'écriture snake) : \n");
    strin.push_str(&"test");
    strin.push_str(".json");
    fs::write(strin, serialized).expect("ça n'a pas écrit");
}
///fonction global reprodusant une intéraction pour les tests
pub fn deseriatest() -> Vec<MusicFile> {
    let vecteur: Vec<MusicFile> =
        serde_json::from_str(&fs::read_to_string("test.json").expect("erreur de lecture\n"))
            .expect("erreur de désérialisation\n");
    vecteur
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonctminim::convertstringtooptioni32;
    use std::path::Path;

    #[test]
    fn testing_seria_deseria() {
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
        serialvectest(&vecteur);

        let vectest=deseriatest();

        assert_eq!(vectest[2].artist=="test" && vectest[1].artist=="1" && vectest[0].artist=="powerwolf",true);
    }
}
