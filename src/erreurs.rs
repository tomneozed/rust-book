// Modules

// Imports
use std::{borrow::BorrowMut, collections::HashMap, io, io::ErrorKind, ops::Div, fs::File, fs};

use io::Read;


pub fn gestion_des_erreurs() {

    println!("RUST BOOK - 9. La gestion des erreurs");

    // Cours 
    panique();

    // Result
    result();

}

fn panique() {
    // panic! = message d'erreur + déroulement et nettoyage de la pile + fermeture du programme
    panic!("crash & burn");
}

fn result() {

    // méthode rudimentaire
    let f = File::open("hello.txt");
    let f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => match erreur.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur lors de la création du fichier {:?}", e),
            },
            autre_erreur =>  panic!("Erreur lors de l'ouverture du fichier {:?}", autre_erreur),
        },
    };

    // méthode de rustacé

    let f = File::open("hello.txt").unwrap_or_else(|erreur| {
        if erreur.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erreur| {
                panic!("Erreur lors de la création du fichier {:?}", erreur);
            })
        } else {
            panic!("Erreur lors de l'ouverture du fichier {:?}", erreur);
        }
    });

    // raccourcis pour faire un panic lors d'une erreur
        // unwrap
        // si la valeur de Result est Ok, unwrap retourne la valeur dans le Ok, 
        // sinon -> panic!
        let f = File::open("hello.txt").unwrap();
        
        // expect
        // Permet de définir le message d'erreur du panic
        let f = File::open("hello.txt").expect("Echec à l'ouverture de hello.txt");

    // propager les erreurs

        // méthode rudimentaire

        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");
        
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };
        
            let mut s = String::new();
        
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        // méthode avancée

        fn read_username_from_file_2() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // Le '?' placé derriere une valeur Result permet de retourner 
        //      Si Ok  -> on retourne Ok(v)
        //      Si Err -> on retourne Err(e)

        // méthode de rustacé

        fn read_username_from_file_3() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }

        // méthode de rustacé ++
        fn read_username_from_file_4() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
}