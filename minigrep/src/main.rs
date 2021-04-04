use std::env;
// use std::fs;
use std::process;
// use std::error::Error;
use minigrep::Config;

fn main() {
    // Lire les valeurs des arguments
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // // Enregistrer les valeurs des arguments dans des variables

    // let recherche = &args[1];
    // let nom_fichier = &args[2];

    // println!("On recherche : {}", recherche);
    // println!("Dans le fichier : {}", nom_fichier);
    // // Lire un fichier

    // let contenu = fs::read_to_string(nom_fichier)

    //     .expect("Quelque chose s'est mal passé lors de la lecture de votre fichier");
    
    // println!("Dans le texte: \n{}", contenu);

    // Réarrangement du code 

    // let (recherche, nom_fichier) = interpreter_config(&args);

    // println!("On recherche : {}", recherche);
    // println!("Dans le fichier : {}", nom_fichier);

    // let contenu = fs::read_to_string(nom_fichier)
    //     .expect("Quelque chose s'est mal passé lors de la lecture de votre fichier");

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    // println!("On recherche : {}", config.recherche);
    // println!("--- Dans le fichier : {} ---", config.nom_fichier);

    if let Err(e) = minigrep::run(config){
        eprintln!("Erreur explicative : {}", e);
        process::exit(1);
    }


}

// struct Config {
//     recherche: String,
//     nom_fichier: String,
// }

// fn interpreter_config(args: &[String]) -> Config {
//     let recherche = args[1].clone();
//     let nom_fichier = args[2].clone();
    
//     Config { recherche, nom_fichier }
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("Il n'y a pas assez d'arguments !")
//         }
//         let recherche = args[1].clone();
//         let nom_fichier = args[2].clone();
        
//         Ok(Config { recherche, nom_fichier })
//     }
// }


// fn run(config: Config) -> Result<(), Box<dyn Error>> {      //dyn = dynamique -> un type qui implémente le trait Error
//     let contenu = fs::read_to_string(config.nom_fichier)?;
    
//     println!("Dans le texte :\n{}", contenu);
//     Ok(())
// }