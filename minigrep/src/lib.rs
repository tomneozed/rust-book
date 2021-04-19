use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub recherche: String,
    pub nom_fichier: String,
    pub sensible_casse: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("Il n'y a pas assez d'arguments !")
    //     }
    //     let recherche = args[1].clone();
    //     let nom_fichier = args[2].clone();

    //     // let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();
    //     let sensible_casse = false;
        
    //     Ok(Config { recherche, nom_fichier, sensible_casse })
    // }
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let recherche = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de chaine de caracteres"),
        };
        let nom_fichier = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de nom de fichier"),
        };

        let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();
        
        Ok(Config { recherche, nom_fichier, sensible_casse })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {      //dyn = dynamique -> un type qui implémente le trait Error
    let contenu = fs::read_to_string(config.nom_fichier)?;

    let resultats = if config.sensible_casse {
        rechercher(&config.recherche, &contenu)
    } else {
        rechercher_insensible_casse(&config.recherche, &contenu)
    };

    for ligne in resultats {
        println!("{}", ligne);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultat() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape";
        assert_eq!(
            vec!["sécurité, rapidité, productivité."], 
            rechercher(recherche, contenu)
        );
    }
    #[test]
    fn insensible_casse() {
        let recherche = "rUsT";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique";
        assert_eq!(
            vec!["Rust:", "C'est pas rustique"], 
            rechercher_insensible_casse(recherche, contenu)
        );
    }
}

// pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
//     let mut resultats = Vec::new();

//     for ligne in contenu.lines() {
//         if ligne.contains(recherche) {
//             resultats.push(ligne);
//         }
//     }
//     resultats
// }

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    contenu
        .lines()
        .filter(|lignes| ligne.contains(recherche))
        .collect()
}

pub fn rechercher_insensible_casse<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let recherche = recherche.to_lowercase();
    let mut resultats = Vec::new();

    for ligne in contenu.lines() {
        if ligne.to_lowercase().contains(&recherche) {
            resultats.push(ligne);
        }
    }
    resultats
}