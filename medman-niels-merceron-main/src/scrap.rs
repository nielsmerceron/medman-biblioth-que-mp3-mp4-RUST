use crate::allfile::MusicFile;
use crate::fonctminim::{getinfoa, getinfob};
use crate::tag::tagchangem;
use serde_json::Value;

///Scrap cherchant sur musicbrainz des données supplémentaire + changement des tags si la music est dans le répertoire
#[tokio::main]
pub async fn scrap(vecteur: &Vec<MusicFile>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("MyAwesomeTagger/1.2.0")
        .build()?;

    println!("\nScrap sur musicbrainz.org.");

    println!("Entré titre:");
    let buf1 = getinfoa();

    println!("Entré artist :");
    let buf2 = getinfoa();

    let url = "https://musicbrainz.org/ws/2/recording/?query=recording:".to_owned()
        + &buf1
        + " AND artist:"
        + &buf2
        + "&limit=1&fmt=json";

    let result = client.get(&url).send().await?.text().await?;
    let v: Value = serde_json::from_str(&result)?;

    let title = &v["recordings"][0]["title"]
        .to_string()
        .trim_matches('\"')
        .to_string();

    let mut artist = v["recordings"][0]["artist-credit"][0]["name"]
        .to_string()
        .trim_matches('\"')
        .to_string();

    let album = v["recordings"][0]["releases"][0]["title"]
        .to_string()
        .trim_matches('\"')
        .to_string();

    let year = v["recordings"][0]["first-release-date"]
        .to_string()
        .trim_matches('\"')
        .split('-')
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    if v["recordings"][0]["artist-credit"][0]["joinphrase"].is_null() {
        println!(
            "\n  résultat de la recherche :\nTitre : {}\nArtist : {}\nAlbum : {}\n date : {}",
            title, artist, album, year
        );
    } else {
        artist = artist
            + " ft. "
            + &v["recordings"][0]["artist-credit"][1]["name"]
                .to_string()
                .trim_matches('\"')
                .to_string();
    }
    println!(
        "\n résultat de la recherche :\nTitre : {}\nArtist : {}\nAlbum : {}\n date : {}",
        title, artist, album, year
    );
    println!("voulez vous si la music est présente dans le répertoire qu'il complète les tags manquant de la musique? [y/n]");
    if getinfob() == "y" {
        for i in vecteur {
            if i.title == buf1 {
                println!("  changement de tag en cours\n");
                tagchangem(&i.path, &artist, title, &album, &i.genre, &year.to_string());
                println!("changement fini\n");
            }
        }
    }
    Ok(())
}
