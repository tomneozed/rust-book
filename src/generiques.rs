use std::fmt::Display;

pub fn generiques() {

    println!("RUST BOOK - 10. Les types génériques, les traits et les durées de vie");

    // Cours 
    supprimer_les_doublons();
    types_de_donnees_generiques();
    les_traits();
    durees_de_vie();
    generiques_traits_durees_de_vie();

    // Exos

}

fn supprimer_les_doublons() {
    println!("- 10.0 Supprimer les doublons -");
    
    let liste_de_nombres = vec![34, 50, 25, 100, 65];
    
    let resultat = le_plus_grand_i32(&liste_de_nombres);
    println!("Le nombre le plus grand est : {}", resultat);
    
    let liste_de_nombres = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let resultat = le_plus_grand_i32(&liste_de_nombres);
    println!("Le nombre le plus grand est : {}", resultat);
}

// Retourne le plus grand i32 de la liste 'liste' 
// fn le_plus_grand(liste: &[i32]) -> i32 {
//     let mut le_plus_grand = liste[0];

//     for &element in liste {
//         if element > le_plus_grand {
//             le_plus_grand = element;
//         }
//     }
//     le_plus_grand
// }

fn types_de_donnees_generiques() {

    println!("- 10.1 Les types de données génériques -");

    // -------------------- Les lignes ci-dessous renvoient à une erreur --------------------

    // let liste_de_nombres = vec![34, 50, 25, 100, 65];
    // let resultat = le_plus_grand(&liste_de_nombres);
    // println!("Le plus grand nombre est {}", resultat);

    // let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];

    // let resultat = le_plus_grand(&liste_de_caracteres);
    // println!("Le plus grand caractère est {}", resultat);

    // -------------------- Les lignes ci-dessus renvoient à une erreur ---------------------

    println!(r#""#);

    println!("- Définition des structures -");

    println!(r#""#);
    println!("Structure : Point<T>");
    println!(r#""#);
    struct Point<T>{
        x: T,
        y: T,
    }

    let entiers = Point {x: 5, y: 10};
    let flottants = Point {x: 1.0, y: 4.0};
    //let ne_fonctionne_pas = Point {x: 5, y: 4.0};
    println!(" entiers : [x: {:?}, y: {:?}]", entiers.x, entiers.y);
    println!(" flottants : [x: {:?}, y: {:?}]", flottants.x, flottants.y);
    println!(" ne_fonctionnera_pas : [x: {:?}, y: {:?}] <- x et y n'ont pas le même type (i32, f64)", 5, 4.0);
    println!(r#""#);

    println!(r#""#);
    println!("Structure : Point<T,U>");
    println!(r#""#);
    struct MultiPoint<T, U>{
        x: T,
        y: U,
    }

    let multiers = MultiPoint {x: 5, y: 10};
    let multants = MultiPoint {x: 1.0, y: 4.0};
    let fonctionnera = MultiPoint {x: 5, y: 4.0};
    println!(" multiers : [x: {:?}, y: {:?}]", multiers.x, multiers.y);
    println!(" multants : [x: {:?}, y: {:?}]", multants.x, multants.y);
    println!(" fonctionnera : [x: {:?}, y: {:?}] <- x et y n'ont pas le même type (i32, f64)", fonctionnera.x, fonctionnera.y);
    println!(r#""#);

    println!(r#""#);
    println!("- Définitions d'énumérations -");
    println!(r#""#);

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    println!(r#""#);
    println!("- Définitions des méthodes -");
    println!(r#""#);

    //Implémentation générique
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    //Implémentation spécifique f32
    impl Point<f32> {
        fn distance_depuis_lorigine(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    //Implémentation multiple
    impl<T, U> MultiPoint<T, U> {
        fn melange<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
        }
    }

    let p1 = MultiPoint {x: 5, y: 10.4};
    let p2 = MultiPoint {x: "Hello", y: 'c'};
    println!(" p1 : [x: {:?}, y: {:?}]", p1.x, p1.y);
    println!(" p2 : [x: {:?}, y: {:?}]", p2.x, p2.y);

    println!(r#""#);
    println!(" mélange p1 et p2");
    let p3 = p1.melange(p2);
    println!(r#""#);
    
    println!(" p3 : [x: {:?}, y: {:?}]", p3.x, p3.y);
    println!(r#""#);
}

fn le_plus_grand_i32(liste: &[i32]) -> &i32 {
    let mut le_plus_grand = &liste[0];

    for element in liste.iter() {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }

    le_plus_grand
}

fn le_plus_grand_caractere(liste: &[char]) -> &char {
    let mut le_plus_grand = &liste[0];

    for element in liste.iter() {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }

    le_plus_grand
}

// Renvoie une erreur
// fn le_plus_grand<T>(liste: &[T]) -> &T {
//     let mut le_plus_grand = &liste[0];
//     for element in liste {
//         if element > le_plus_grand {
//             le_plus_grand = element;
//         }
//     }
//     le_plus_grand
// }

fn les_traits() {

    println!("- 10.2 Définir les comportements partagés avec les traits -");

    pub trait Resumable {

        // [Partie 1]
        // fn resumer(&self) -> String;

        // [Partie 2]
        fn resumer_auteur(&self) -> String;

        fn resumer(&self,) -> String {
            format!("(lire plus d'éléments de {} ...)", self.resumer_auteur())
        }
    }

    pub struct ArticleDePresse {
        pub titre: String,
        pub lieu: String,
        pub auteur: String,
        pub contenu: String,
    }
    
    // [Partie 1]
    // impl Resumable for ArticleDePresse {
    //     fn resumer(&self) -> String {
    //         format!("{}, par {} ({})",self.titre, self.auteur, self.lieu)
    //     }
    // }

    pub struct Tweet {
        pub nom_utilisateur: String,
        pub contenu: String,
        pub reponse: bool,
        pub retweet: bool,
    }

    // [Partie 1]
    // impl Resumable for Tweet {
    //     fn resumer(&self) -> String {
    //         format!("{} : {}", self.nom_utilisateur, self.contenu)
    //     }
    // }

    // [Partie 2]
    impl Resumable for Tweet {
        fn resumer_auteur(&self) -> String {
            format!("@{}", self.nom_utilisateur)
        }
    }

    let tweet = Tweet {
        nom_utilisateur: String::from("jean"),
        contenu: String::from(" Bien sûr c'est comme ça qu'on fait les bébés"),
        reponse: false,
        retweet: false,
    };

    println!(r#""#);
    println!("1 nouveau tweet : {}", tweet.resumer());
    println!(r#""#);

    println!(r#""#);
    println!("- Des traits en paramètres -");
    println!(r#""#);

    // ----------------------  Traits liés ----------------------

    pub fn notifier(element: &impl Resumable) {
        println!("Flash-info ! {}", element.resumer());
    }

    // ^^^ Equivalents vvv

    // pub fn notifier<T: Resumable>(element: &T) {
    //     println!("Flash-info ! {}", element.resumer());
    // }

    // ------------------- Plusieurs Traits liés -------------------

    // pub fn notifier(element: &(impl Resumable + Affichable)) {
    //     println!("Flash-info ! {}", element.resumer());
    // }

    // ^^^ Equivalents vvv

    // pub fn notifier<T: Resumbale + Affichable>(element: &T) {
    //     println!("Flash-info ! {}", element.resumer());
    // }

    // ------------------- Traits liés avec where -------------------

    // Marche pas comme ça
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    //     0
    // }

    // fn some_function<T, U>(t: &T, u: &U) -> i32 
    // where T: Display + Clone,
    //       U: Clone + Debug
    //       {

    //       }

    // fn retourne_resumable(estArticle: bool) -> impl Resumable {
    //     if estArticle {
    //         ArticleDePresse {
    //             titre: String::from("Les pinguins ont gagné la Stanley Cup ChampionShip !"),
    //             lieu: String::from("Pittsburg, PA, USA"),
    //             auteur: String::from("Iceburgh"),
    //             contenu: String::from("Les pinguins sont noirs vu du dessus et blanc vu du dessous,
    //             ce qui permet de les camoufler des prédateurs comme des proies"),
    //         }
    //     } else {
    //         Tweet {
    //             nom_utilisateur: String::from("jean"),
    //             contenu: String::from(" Bien sûr c'est comme ça qu'on fait les bébés"),
    //             reponse: false,
    //             retweet: false,
    //         }
    //     }
    // }

    println!(r#""#);
    println!(" Le plus grand & Traits liés");
    println!(r#""#);

    fn le_plus_grand<T: PartialOrd + Copy>(liste: &[T]) -> T {
        let mut le_plus_grand = liste[0];

        for &element in liste {
            if element > le_plus_grand {
                le_plus_grand = element;
            }
        }
        le_plus_grand
    }

    let liste_de_nombres = vec![34, 50, 25, 100, 65];
    
    let resultat = le_plus_grand(&liste_de_nombres);

    println!(r#""#);
    println!("liste de nombres : {:?}", liste_de_nombres);
    println!("resultat : {:?}", resultat);
    println!(r#""#);
    
    let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];

    let resultat = le_plus_grand(&liste_de_caracteres);

    println!(r#""#);
    println!("liste de caracteres : {:?}", liste_de_caracteres);
    println!("resultat : {:?}", resultat);
    println!(r#""#);


    println!(r#""#);
    println!(" Traits liés pour implémenter les méthodes");
    println!(r#""#);

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {x, y}
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn affiche_comparaison(&self) {
            if self.x >= self.y {
                println!("Le plus grand élément est x = {}", self.x);
            } else {
                println!("Le plus grand élément est y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(1, 8);

    pair.affiche_comparaison();
    
    println!(r#""#);
}

fn durees_de_vie() {
    println!("- 10.3 La conformité des références avec les durées de vie -");
    println!("");
    durees_de_vie_generiques_dans_les_fonctions();
    durees_de_vie_et_signature_des_fonctions();
    durees_de_vie_et_definition_des_structures();
    durees_de_vie_et_definition_des_methodes();
    duree_de_vie_statique();
}

fn durees_de_vie_generiques_dans_les_fonctions() {
    println!("");
    println!("- Les durées de vie génériques dans les fonctions -");
    println!("");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let resultat = la_plus_longue(string1, string2);
    // println!("la plus longue : {}", resultat);


    // fn la_plus_longue(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     }else {
    //         y
    //     }
    // }

    // &i32        // une référence
    // &'a i32     // une référence avec une durée de vie explicite
    // &'a mut i32 // une référence mutable avec une durée de vie explicite






}

fn durees_de_vie_et_signature_des_fonctions() {
    println!("");
    println!("- Les annotations de durées de vie dans les signatures des fonctions -");
    println!("");

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let resultat = la_plus_longue(string1.as_str(), string2.as_str());
    println!("la plus longue : {}", resultat);

    fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }



    
}

fn durees_de_vie_et_definition_des_structures() {
    println!("");
    println!("- L'ajout des durées de vies dans les définitions des structures -");
    println!("");

    struct ExtraitImportant<'a> {
        partie: &'a str,    // une instance de ExtraitImportant ne peut pas vivre plus longtemps que la ref de son champ partie
    }

    let roman = String::from("Appelez-moi Ismaël. Il y a quelques années ...");
    let premiere_phrase = roman.split('.')
        .next()
        .expect("Impossible de trouver un '.'");
    let i = ExtraitImportant { partie: premiere_phrase };
}

fn durees_de_vie_et_definition_des_methodes() {
    println!("");
    println!("- Informations de durée de vie dans les définitions des méthodes -");
    println!("");

    struct ExtraitImportant<'a> {
        partie: &'a str,    
    }

    impl<'a> ExtraitImportant<'a> {
        fn niveau(&self) -> i32 {
            3
        }

        fn annoncer_et_retourner_partie(&self, annonce: &str) -> &str {
            println!("Votre attention s'il vous plait: {}", annonce);
            self.partie
        }
    }
}

fn duree_de_vie_statique() {
    println!("");
    println!("- La durée de vie statique -");
    println!("");

    let s: &'static str = "J'ai une durée de vie statique";     // Durée de vie = totalité du programme -> /!\
}

fn generiques_traits_durees_de_vie() {
    println!("");
    println!("- Les paramètres de type génériques, les traits liés, et les durées de vies ensemble -");
    println!("");

    fn la_plus_longue_avec_annonce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
        println!("Annonce ! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}












