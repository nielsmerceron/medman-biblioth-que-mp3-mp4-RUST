use std::io;
use std::path::Path;

///les fonctions de cette section ne sont pas obligatoire, c'est juste pour que ça soit plus propre

///fonction prenant une option<&str> pour la convertir en string
pub fn convertoptost(conv: Option<&str>) -> String {
    if conv.is_some() {
        let s: Option<&str> = conv;
        let a: Option<String> = s.map(str::to_string);
        match a {
            Some(a) => a,
            None => "".to_string(),
        }
    } else {
        "".to_string()
    }
}
/// fonction convertissant un path buf en string
pub fn convertpathbuftostring(chemin: &Path) -> String {
    let retour = convertoptost(chemin.to_str());
    retour
}

/// fonction convertissant un optioni32 en string
pub fn convertoptioni32toi32(change: Option<i32>) -> String {
    match change {
        Some(w) => w.to_string(),
        None => "".to_string(),
    }
}

///fonction convertissant un string en option i32
pub fn convertstringtooptioni32(change: &str) -> Option<i32> {
    Some(change.parse::<i32>().unwrap())
}

///fonction convertissant un string en u32
pub fn convertstringtoi32(change: &str) -> i32 {
    change.parse::<i32>().unwrap_or(0)
}

///fonction attendant une ligne écrite par l'utilisateur et qui supprime le "\n"
pub fn getinfoa() -> String {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line
}

///fonction attendant une ligne écrite par l'utilisateur et qui supprime le "\n" et les espaces (adapté pour les noms de fichier)
pub fn getinfob() -> String {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace());
    line
}
