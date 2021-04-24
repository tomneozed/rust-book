use std::fmt::Display;
use std::thread;
use std::time::Duration;

pub fn langages_fonctionnels() {

    println!("RUST BOOK - 13. Les fonctionnalités des langages fonctionnels : les itérateurs et les fermetures");

    // Cours 
    les_fermetures();
    les_iterateurs();
    amelioration_projet_entree_sortie();
    comparaison_boucles_iterateurs();

    // Exos

}

fn les_fermetures(){
    println!("- 13.1 Les fermetures : fonctions anonymes qui peuvent utiliser leur environnement -");
    println!(r#""#);
    println!("- Créer une abstraction de comportement avec une fermeture -");
    println!(r#""#);

    let valeur_utilisateur_simule = 10;
    let nombre_aleatoire_simule = 7;

    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);

    println!(r#""#);
    println!("- L'inférence de type et l'annotation des fermetures -");
    println!(r#""#);

    // Les 4 lignes suivantes sont équivalentes
    // fn  ajouter_un_v1   (x: u32) -> u32 { x + 1 }
    // let ajouter_un_v2 = |x: u32| -> u32 { x + 1 };
    // let ajouter_un_v3 = |x|             { x + 1 };
    // let ajouter_un_v4 = |x|               x + 1  ;


    println!(r#""#);
    println!("- Stockage des fermetures avec des paramètres génériques et le trait Fn -");
    println!(r#""#);

    println!(r#""#);
    println!("- Limitations de l'implémentation de Cache -");
    println!(r#""#);

    println!(r#""#);
    println!("- Capturer l'environnement avec les fermetures -");
    println!(r#""#);

    // fonctionne avec les fermetures
    let x = 4;
    let egal_a_x = |z| z ==x;
    let y = 4;
    assert!(egal_a_x(y));

    // ne fonctionne pas avec les fonctions
    // let x = 4;
    // fn egal_a_x(z: i32) -> bool {
    //     z == x
    // }
    // let y = 4;
    // assert!(egal_a_x(y));

    let x = vec![1, 2, 3];
    let egal_a_x = move |z| z == x;
    // println!("On ne peut pas utiliser x : {:?} ici", x);
    let y = vec![1, 2, 3];
    assert!(egal_a_x(y));
}

fn les_iterateurs(){
    println!("- 13.2 Traiter une série d'éléments avec un itérateur -");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("On a : {}", val);
    }

    println!(r#""#);
    println!("- Le trait Iterator et la méthode next -");
    println!(r#""#);

    // Voir tests.rs - demo_iterateur

    println!(r#""#);
    println!("- Les méthodes qui consomment un itérateur -");
    println!(r#""#);

    // Voir tests.rs - iterator_sum

    println!(r#""#);
    println!("- Méthodes qui produisent d'autres itérateurs -");
    println!(r#""#);

    let v1: Vec<i32> = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1 );
    // la méthode map() créé un nouvel itérateur et la methode collect() consomme cet itérateur pour créer un nouveau vecteur
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![1, 2, 3]);

    println!(r#""#);
    println!("- Utilisation de fermetures capturant leur environnement -");
    println!(r#""#);

    // Voir tests.rs - filtres_par_pointure

    println!(r#""#);
    println!("- Créer nos propres itérateurs avec le trait Iterator -");
    println!(r#""#);

    // cf Compteur vvv

    println!(r#""#);
    println!("- Utiliser la méthode next de notre Itérateur Compteur -");
    println!(r#""#);

    // Voir tests.rs - appel_direct_a_next

    println!(r#""#);
    println!("- Utiliser d'autres méthodes du trait Iterator -");
    println!(r#""#);

    // Voir test.rs - utilisation_des_autres_methodes_du_trait_iterator
}

fn amelioration_projet_entree_sortie(){
    println!("- 13.3 Amélioration de notre projet d'entrée/sortie -");
    // Voir minigrep/src
}

fn comparaison_boucles_iterateurs(){
    println!("- 13.4 Comparaison des performances : les boucles et les itérateurs -");

    let tampon: &mut [i32];
    let coefficients: [i64; 12];
    let decalage: i16;

    // for i in 12..tampon.len() {
    //     let prediction = coefficients.iter()
    //                                 .zip(&tampon[i - 12..i])
    //                                 .map(|(&c, &s)| c * s as i64)
    //                                 .sum::<i64>() >> decalage;
    //     let delta = tampon[i];
    //     tampon[i] = prediction as i32 + delta;
    // }

}

pub fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("calcul très lent ...");
    thread::sleep(Duration::from_secs(2));
    intensite
}

// pub fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
//     if intensite < 25 {
//         println!("Aujourd'hui, faire {} pompes !", simuler_gros_calcul(intensite));
//         println!("Ensuite, faire {} abdos !", simuler_gros_calcul(intensite));
//     } else {
//         if nombre_aleatoire == 3 {
//             println!("Faites un pause aujourd'hui, rapellez vous de bien vous hydrater");
//         } else {
//             println!("Aujourd'hui, courez {} minutes !", simuler_gros_calcul(intensite));
//         }
//     }
// }

// pub fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
//     // let resultat_lent = simuler_gros_calcul(intensite);
//     let fermeture_lente = |nombre| {
//         println!("calcul très lent ...");
//         thread::sleep(Duration::from_secs(2));
//         nombre
//     };
//     if intensite < 25 {
//         // println!("Aujourd'hui, faire {} pompes !", resultat_lent);
//         // println!("Ensuite, faire {} abdos !", resultat_lent);
//         println!("Aujourd'hui, faire {} pompes !", fermeture_lente(intensite));
//         println!("Ensuite, faire {} abdos !", fermeture_lente(intensite));
//     } else {
//         if nombre_aleatoire == 3 {
//             // println!("Faites un pause aujourd'hui, rapellez vous de bien vous hydrater");
//         } else {
//             // println!("Aujourd'hui, courez {} minutes !", resultat_lent);
//             println!("Aujourd'hui, courez {} minutes !", fermeture_lente(intensite));
//         }
//     }
// }

fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    let mut resultat_lent = Cache::new(|nombre| {
        println!("calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    });

    if intensite < 25 {
        println!("Aujourd'hui, faire {} pompes !", resultat_lent.valeur(intensite));
        println!("Ensuite, faire {} abdominaux !", resultat_lent.valeur(intensite));
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                resultat_lent.valeur(intensite)
            );
        }
    }
}

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: None,
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        match self.valeur {
            Some(v) => v,
            None => {
                let v = (self.calcul)(arg);
                self.valeur = Some(v);
                v
            },
        }
    }
}

// #[test]
// fn appel_avec_differentes_valeurs(){
//     println!("TEST");
//     let mut c = Cache::new(|a| a);
//     let v1 = c.valeur(1);
//     let v2 = c.valeur(2);
//     assert_eq!(v2, 2);
// }




