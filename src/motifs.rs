

pub fn motifs() {
    println!("RUST BOOK - 18. Les motifs et le filtrage par motif");

    // Cours
    // endroits_des_motifs();
    // la_refutabilite();
    syntaxe_des_motifs();
    
    // Exos

}

// -------------------------- RUST BOOK - 18.1 --------------------------

fn endroits_des_motifs() {

    let couleur_favotite: Option<&str> = None;
    let on_est_mardi = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(couleur) = couleur_favotite {
        println!("Utilisation de votre couleur favorite, {}, comme couleur de fond", couleur);
    } else if on_est_mardi {
        println!("Mardi c'est le jour du verre !");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Utilisation du violet comme couleur de fond");
        } else {
            println!("Utilisation de l'orange comme couleur de fond");
        }
    } else {
        println!("Utilisation du bleu comme couleur de fond");
    }

    let mut pile = Vec::new();

    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(donne_du_haut) = pile.pop() {
        println!("{}", donne_du_haut);
    }

    let v = vec!['a', 'b', 'c'];

    for (indice, valeur) in v.iter().enumerate() {
        println!("{} est à l'indice {}", valeur, indice);
    }

    // Ne fonctionnera pas 
    // let (x, y) = (1, 2, 3); 

    let point = (3, 5);
    afficher_coordonnees(&point);
}

fn afficher_coordonnees(&(x, y): &(i32, i32)) {
    println!("Coordonnees actuelles : ({}; {})", x, y);
}

// -------------------------- RUST BOOK - 18.2 --------------------------

fn la_refutabilite() {

    // if let Some(x) = une_option_quelconque {
    //     println!("{}", x);
    // }

    if let x = 5 {
        println!("{}", x);
    };

}

// -------------------------- RUST BOOK - 18.3 --------------------------

fn syntaxe_des_motifs() {

    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("On a 50"),
    //     Some(y) => println!("Correspondance : {:?}", x),
    //     _ => println!("Cas par défaut : x = {:?}", x),
    // }

    // println!("A la fin : x = {:?}, y = {:?}", x, y);

    // let x = 5;

    // match x {
    //     1..=5 => println!("de un à cinq"),
    //     _ => println!("quelque chose d'autre"),
    // }

    // let x = 'c';

    // match x {
    //     'a'..='j' => println!("lettre ASCII du début"),
    //     'k'..='z' => println!("lettre ASCII de la fin"),
    //     _ => println!("autre chose"),
    // }

    // let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);

    // match p {
    //     Point { x, y: 0 } => println!("Sur l'axe x à la position {}", x),
    //     Point { x: 0, y } => println!("Sur l'axe y à la position {}", y),
    //     Point { x, y } => println!("Sur aucun des axes : ({}, {})", x, y),
    // }

    // let msg = Message::ChangerCouleur(Couleur::Tsv(0, 160, 255));

    // match msg {
    //     Message::Quitter => {
    //         println!("La variante Quitter n'a pas de données à déstructurer.")
    //     }
    //     Message::Deplacer { x, y } => {
    //         println!("Déplacement de {} sur l'axe x et de {} sur l'axe y", x, y);
    //     }
    //     Message::Ecrire(text) => println!("Message textuel : {}", text),
    //     Message::ChangerCouleur(Couleur::Rvb(r, g, b)) => println!("Changement des taux de rouge à {}, de vert à {} et de bleu à {}", r, g, b),
    //     Message::ChangerCouleur(Couleur::Tsv(r, g, b)) => println!("Changement des taux de teinte à {}, de saturation à {} et de valeur à {}", r, g, b),
    //     _ => (),
    // }

    // let ((pieds, pouces), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});

    // let mut valeur_du_reglage = Some(5);
    // let nouvelle_valeur = Some(10);

    // match (valeur_du_reglage, nouvelle_valeur) {
    //     (Some(_), Some(_)) => {
    //         println!("Vous ne pouvez pas écraser une valeur déjà existante");
    //     }
    //     _ => {
    //         valeur_du_reglage = nouvelle_valeur;
    //     }
    // }

    // println!("Réglage : {:?}", valeur_du_reglage);

    let nombres = (2, 4, 8, 16, 32);

    match nombres {
        // (premier, _, troisieme, _, cinquieme) => {
        //     println!("Voici quelques nombre {} {} {}", premier, troisieme, cinquieme);
        // },
        (premier, .., dernier) => {
            println!("Voici quelques nombre {} {}", premier, dernier);
        },
        //  vv          vv : trop ambigu -> ne compilera pas
        // (.., second, ..) => {
        //     println!("Voici quelques nombre {}", second);
        // },
    }

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Salutations !"));

    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("J'ai trouvé les caracteres");
    }

    // println!("{:?}", s);
    println!("{:?}", s);

    let origine = Point {x:0, y:0, z:0 };

    match origine {
        Point { x, ..} => println!("x vaut {}", x),
    }

    let nombre = Some(4);

    match nombre {
        Some(x) if x < 5 => println!("Moins que 5 : {}", x),
        Some(x) => println!("x: {}", x),
        None => (),    
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Nous obtenons 50"),
        Some(n) if n == y => println!("Nous avons une correspondance, n = {}", n),
        _ => println!("Cas par défaut, x = {:?}", x),
    }

    println!("Au final : x = {:?}, y = {}", x, y);

    let z = false;

    match y {
        4 | 5 | 6 if z => println!("yes"),
        _ => println!("No"),
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => println!("Nous avons touvé un ID dans l'intervalle : {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Nous avons trouvé un ID dans un autre intervalle")
        }
        Message2::Hello { id } => println!("Nous avons trouvé un autre ID : {}", id),
    }
}

// struct Point {
//     x: i32,
//     y: i32,
// }

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(Couleur),
}

enum Couleur {
    Rvb(i32, i32, i32),
    Tsv(i32, i32, i32),
}

fn fonction(_: i32, y: i32) {
    println!("Ce code utilise uniquement le paramètre y : {}", y);
}

fn main() {
    fonction(3, 4);
}

enum Message2 {
    Hello {id: i32},
}