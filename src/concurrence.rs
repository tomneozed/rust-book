use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::rc::Rc;


pub fn concurrence() {
    println!("RUST BOOK - 16. La concurrence sans craintes");

    // Cours
    // les_taches();
    // les_messages();
    partage_d_etat();
    // etendre_la_concurrence();

    // Exos

}

// -------------------------- RUST BOOK - 16.1 --------------------------

fn les_taches() {
    // let manipulateur = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Bonjour n°{} à partir de la nouvelle tâche ! ", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // manipulateur.join().unwrap();

    // for i in 1..5 {
    //     println!("Bonjour n°{} à partir de la tâche principale ! ", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let v = vec![1, 2, 3];

    let manipulateur = thread::spawn(move || {
        println!("Voici un vecteur : {:?}", v);
    });

    manipulateur.join().unwrap();
}

// -------------------------- RUST BOOK - 16.2 --------------------------

fn les_messages() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let valeur = String::from("salut");
    //     tx.send(valeur).unwrap();
    //     // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv erreur car la valeur a été déplacée ligne du dessus
    //     // println!("valeur vaut : {}", valeur);
    // });

    // let recu = rx.recv().unwrap();
    // println!("On a reçu : {}", recu);


    // ---------------------------------------------------------

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let valeurs = vec![
            String::from("salutations"),
            String::from("à partir"),
            String::from("de la"),
            String::from("nouvelle tâche"),
        ];

        for valeur in valeurs {
            tx1.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let valeurs = vec![
            String::from("encore plus"),
            String::from("de messages"),
            String::from("pour"),
            String::from("vous"),
        ];

        for valeur in valeurs {
            tx.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recu in rx {
        println!("On a reçu : {}", recu);
    }    
}

// -------------------------- RUST BOOK - 16.3 --------------------------

fn partage_d_etat() {
    let m = Mutex::new(5);
    {
        let mut nombre = m.lock().unwrap();
        *nombre = 6;
    }
    println!("m={:?}", m);

    let compteur = Arc::new(Mutex::new(0));
    let mut manipulateurs = vec![];
    
    for _ in 0..10 {
        let compteur = Arc::clone(&compteur);
        let manipulateur = thread::spawn(move || {
            let mut nombre = compteur.lock().unwrap();

            *nombre += 1;
        });
        manipulateurs.push(manipulateur);
    }

    for manipulateur in manipulateurs {
        manipulateur.join().unwrap();
    }

    println!("resultat : {}", *compteur.lock().unwrap());

}