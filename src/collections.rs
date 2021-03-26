use std::{borrow::BorrowMut, collections::HashMap, io, ops::Div};

pub fn collections() {
    
    println!("RUST BOOK - 8. Les collections Standard");

    // Cours
    vecteurs();
    lesStrings();
    tableDeHachage();
    
    // Exercices
    exercicesCollections()
}

// https://doc.rust-lang.org/std/vec/struct.Vec.html
fn vecteurs(){

    // Créer un nouveau vecteur
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];


    // Modification d'un vecteur

    let mut v = Vec::new();
    
    v.push(5); //v[0]
    v.push(6); //v[1]
    v.push(7); //v[2]
    v.push(8); //v[3]

    // Lire les éléments d'un vecteur

    let troisieme: &i32 = &v[2];
    println!("Le 3eme élément est {}", troisieme); 

    match v.get(2) {
        Some(troisieme) => println!("Le 3eme élément est {}", troisieme),
        None => println!("Il n'y a pas de 3eme élément"),
    }

    //let existe_pas = &v[100];       // panic -> programme plante
    let existe_pas2 = v.get(100);    // retourne None


    // Itérer sur les valeurs d'un vecteur

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    //Utiliser une énumération pour stocker différents types

    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let ligne = vec![
        Cellule::Int(3),
        Cellule::Text(String::from("bleu")),
        Cellule::Float(10.12),
    ];

} // <- v1, v2 et v sortent de la portée ici : ils sont supprimés et les valeurs qu'ils contiennent aussi

fn lesStrings(){
    
    // /!\ Les strings sont codés en UTF-8

    // Créer une nouvelle String

    let mut s = String::new();

    let donnee = "contenu initial";

    let s1 = donnee.to_string();

    let s2 = "contenu initial".to_string();

    let s3 = String::from("contenu initial");


    // Modifier une String

    s.push_str(donnee);      // push_str() accepte les strings

    println!("s: {}", s);   

    s.push('c');                // push accepte les char uniquement

    println!("s: {}", s);


    // Concaténation avec '+' ou la macro format!

    let s4 = String::from("Hello ");
    let s5 = String::from("World !");

    println!("s4: {}", s4);

        // L'opérateur '+' utilise la méthode add(self, s: &str) -> String 

    let s6 = s4 + &s5;  // s4 est déplacé dans s6 et ne sera donc plus utilisable

    println!("s5: {}", s5);
    println!("s6: {}", s6);


    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

        // La macro format! fonctionne comme println! mais retourne une String plutôt que de l'afficher
        // de plus, elle ne prend pas possession de ses paramètres donc ils restent utilisables
    let s10 = format!("{}-{}-{}", s7, s8, s9);

    println!("s10: {}", s10);


    // L'indexation des strings

    let hello = "Здравствуйте";
    let answer = &hello[0..4];      // contient les 4 premiers octets de la CdC

    println!("answer : {}", answer);


    // Parcourir les chaines de caractères
    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);              // Impression des 18 octets qui constituent cette String
    }
}

fn tableDeHachage() {

    // Création d'une table de hachage
    let mut points = HashMap::new();

    points.insert(String::from("bleue"), 10);
    points.insert(String::from("rouge"), 50);

    let equipes = vec![String::from("bleue"), String::from("rouge")];
    let points_initiaux = vec![10, 50];

    let mut points: HashMap<_, _> = equipes.into_iter().zip(points_initiaux.into_iter()).collect();

    // Tables de hachage et possession

        // pour les types qui implémentent le Trait Copy (comme les i32), les valeurs sont copiées dans la table
        // pour les valeurs possédées (comme les String), les valeurs seront déplacées (la table sera propriétaire de ces valeurs)

    let nom_champ = String::from("Couleur favorite");
    let valeur_champ = String::from("Bleu");
    
    let mut table = HashMap::new();
    table.insert(nom_champ, valeur_champ);

        // nom_champ et valeur_champ ne sont plus en vigueur à partir d'ici, essayez de
        // les utiliser et vous verrez l'erreur du compilateur que vous obtiendrez !


    // Accéder aux valeurs dans une  table de hachage

    let nom_equipe = String::from("bleue");
    let points_bleu = points.get(&nom_equipe);


    // Modifier une table de hachage

    println!("{:?}", points);
    points.insert(String::from("bleue"), 33);
    println!("{:?}", points);

    // Ajouter une valeur si la clé n'a pas de valeur

    points.entry(String::from("jaune")).or_insert(50);
    println!("{:?}", points);


    // Modifier une valeur en fonction de l'ancienne valeur

        // ici on compte le nombre de fois que chque mot apparaît dans le texte

    let texte = "bonjour le monde , magnifique monde";

    let mut tableau = HashMap::new();

    for mot in texte.split_whitespace() {
        let compteur = tableau.entry(mot).or_insert(0);
        *compteur += 1;
    }

    println!("{:?}", tableau);

}

fn exercicesCollections() {
    exercice1();
    exercice2();
    exercice3();
}

// A partir d'une liste d'entiers, utiliser un vecteur et retourner la moyenne, la médiane et la valeur modale
fn exercice1(){

    println!();
    println!("--- Exercice 1 --- ");
    println!("- moy, med & mod - ");

    let liste_entiers= vec![1,2,3,5,4,2,1,3,6,9,8,7,4,2,1,3];

    let moyenne = get_moy(&liste_entiers);

    println!("la moyenne est de : {}", moyenne);

    let mediane = get_med(&liste_entiers);

    println!("la mediane est : {}", mediane);

    let modale = get_modale(&liste_entiers);

    println!("la modale est : {:?}", modale);    
}

fn get_moy(entree:&Vec<i32>) -> i32 {

    let mut total = 0;
    let mut size = 0;

    for i in entree {
        total += i;
        size += 1;
    }

    let moy = total / size;
    
    return moy;
}

fn get_med(entree:&Vec<i32>) -> i32 {
    let mediane = &entree[entree.len()/2];
    *mediane
}

fn get_modale(entree:&Vec<i32>) -> HashMap<&i32, i32> {
    
    let mut tableau = HashMap::new();

    for nbr in entree {
        let compteur = tableau.entry(nbr).or_insert(0);
        *compteur += 1;
    }
    tableau
}

//Convertir les CdC en Louchébem : la premiere lettre est remplacée par un 'l' et déplacée à la fin du mot et on ajoute le suffixe "em"
// "bonjour" ---> lonjourbem
fn exercice2() {

    println!();
    println!("--- Exercice 2 --- ");
    println!("- Convertir les chaines de caractères en AY - ");

    let cdc = String::from("bonjour");
    let mut iterator = 0;
    let mut premiere_lettre: char = '_';

    let mut nouveau_mot= String::new();

    if commence_par_voyelle(&cdc) {
        let cdc2 = cdc.clone();
        nouveau_mot.push_str(&cdc2);
        nouveau_mot.push_str(&String::from("-hay"));
        println!("Nouveau mot : {}",nouveau_mot);
    } else {
        nouveau_mot.push('l');
        for c in cdc.chars() {
            if iterator == 0 {
                premiere_lettre = c;
            } else {
                nouveau_mot.push(c);
            }
            iterator += 1;
        }
        nouveau_mot.push('-');
        nouveau_mot.push(premiere_lettre);

        nouveau_mot.push_str("ay");    
    }
    println!("{} ----> {}", cdc,  nouveau_mot);
}

fn is_voyelle(c:char) -> bool {
    if c == 'a' ||
       c == 'e' ||
       c == 'i' ||
       c == 'o' ||
       c == 'u' ||
       c == 'y' ||
       c == 'A' ||
       c == 'E' ||
       c == 'I' ||
       c == 'O' ||
       c == 'U' ||
       c == 'Y'  {
            return true;
        }
    false
}

fn commence_par_voyelle(str:&str) -> bool {
    if str.starts_with('a') ||
       str.starts_with('e') ||
       str.starts_with('i') ||
       str.starts_with('o') ||
       str.starts_with('u') ||
       str.starts_with('y') ||
       str.starts_with('A') ||
       str.starts_with('E') ||
       str.starts_with('I') ||
       str.starts_with('O') ||
       str.starts_with('U') ||
       str.starts_with('Y')  {
            return true;
        }
    false
}

// Creer une interface textuelle pour permettre à un utilisateur d'ajouter les noms d'employés dans un département d'une entreprise
// exemple : "Ajouter Sarah au Bureau d'Etudes" ou "Ajouter Amir au Service commercial"
// Donner la possibilité à l'utilisateur de récupérer une liste de toutes les personnes dans un département,
// trié par département et classés dans l'ordre alphabetique dans tous les cas
fn exercice3() {

    println!();
    println!("--- Exercice 3 --- ");
    println!("- Employés des Départements -");

    let mut menu = String::new();
    let mut employes_departements = HashMap::new();

    while menu != String::from("3") {
        menu.clear();
        afficher_menu();

        io::stdin()
            .read_line(&mut menu)
            .unwrap();
        menu = String::from(menu.trim());

        // 1. Ajouter employé
        if String::from(&menu) == String::from("1") {
            // L'utilisateur ajoute un employé à un département
            println!("Veuillez ajouter un employé dans un Département.");

            let mut entree = String::new();
            let mut cpt = 0;
            let mut nom = String::new();
            let mut departement = String::new();
        
            io::stdin()
                .read_line(&mut entree)
                .expect("Échec de la lecture de l'entrée utilisateur");
        
            for mot in entree.split_whitespace() {
                if cpt == 1 {
                    nom = String::from(mot);
                }
                if cpt > 2 {
                    if mot.len()>1  {
                        departement.push_str(mot);
                        departement.push(' ');
                    }
                }
                cpt +=1;
            }
            employes_departements.insert(nom, departement);

        } 
        // 2. Lister les employés
        if String::from(&menu) == String::from("2") && !employes_departements.is_empty() {
            
            // Triage par ordre alphabetique : il nous faut un vecteur
            let mut triee: Vec<_> = employes_departements.iter().collect();
            
            triee.sort_by_key(|a| a.0);

            // Affichage de la liste les employés
            for (key, value) in triee.iter() {
                println!("{} travaille ici : {}", key, value);
            }
        } 
        // 2. Lister les employés et liste vide
        else if String::from(&menu) == String::from("2") {
            println!("Pas d'employé enregistré !");
        }
        // 3. Quitter
        if String::from(&menu) == String::from("3") {
            println!("A tchao bonsoir !");
        }
    }
    
}

fn afficher_menu() {
    println!("");
    println!("------- Menu -------");
    println!("");
    println!("1. Ajouter un employé à un département");
    println!("2. Lister les employés");
    println!("3. Quitter");
    println!("");
}