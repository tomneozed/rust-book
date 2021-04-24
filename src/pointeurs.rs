use self::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub fn pointeurs_intelligents() {
    println!("RUST BOOK - 15. Les pointeurs intelligents");

    // Cours
    // les_box();
    // trait_deref();
    // trait_drop();
    // les_rc();
    // les_refCell();
    boucles_de_reference();

    // Exos
}

// -------------------------- RUST BOOK - 15.1 --------------------------

// fn les_box() {
//     println!("RUST BOOK - 15.1 Utiliser Box<T> pour pointer sur des données présentes sur le tas ");

//     let b = Box::new(5);
//     println!("b = {}", b);
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    // Cons(i32, Rc<List>),
    // Cons(Rc<RefCell<i32>>, Rc<List>),
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

// -------------------------- RUST BOOK - 15.2 --------------------------

fn trait_deref() {
    println!("RUST BOOK - 15.2 Considérer les pointeurs intelligents comme des références grâce au trait Deref ");
    
    let x = 5;
    let y = MaBoite::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MaBoite::new(String::from("Rust"));
    saluer(&m);
    // ou 
    // saluer(&(*m)[..]));
}

struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}

impl<T> Deref for MaBoite<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn saluer(nom: &str) {
    println!("Salutations, {} !", nom);
}

// -------------------------- RUST BOOK - 15.3 --------------------------

fn trait_drop() {
    println!("RUST BOOK - 15.3 Exécuter du code au nettoyage avec le trait Drop ");
    let c = PointeurPerso {
        donnee: String::from("des trucs"),
    };
    let d = PointeurPerso {
        donnee: String::from("d'autres trucs"),
    };
    println!("PointeurPersos créés.");
    drop(c);
    println!("PointeurPerso libéré avant la fin du main.");
}

struct PointeurPerso {
    donnee: String,
}

impl Drop for PointeurPerso {
    fn drop(&mut self) {
        println!("Nettoyage d'un PointeurPerso avec la donnée `{}` !", self.donnee);
    }
}

// -------------------------- RUST BOOK - 15.4 --------------------------

// fn les_rc() {
//     println!("RUST BOOK - 15.4 Rc<T>, le pointeur intelligent qui compte les références ");
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("compteur après la création de a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("compteur après la création de b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("compteur après la création de c = {}", Rc::strong_count(&a));
//     }
//     println!("compteur après que c soit sorti de la portée = {}", Rc::strong_count(&a));
// }

// -------------------------- RUST BOOK - 15.5 --------------------------

fn les_refCell() {
    println!("RUST BOOK - 15.5 RefCell<T> et le motif de mutabilité interne ");

    // let x = 5;
    // // let y = &mut x;

    // let valeur = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&valeur), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *valeur.borrow_mut() += 10;
    // println!("a après les opérations = {:?}", a);
    // println!("b après les opérations = {:?}", b);
    // println!("c après les opérations = {:?}", c);
}

pub trait Messager {
    fn envoyer(&self, msg: &str);
}

pub struct TraqueurDeLimite<'a, T: Messager> {
    messager: &'a T,
    valeur: usize,
    max: usize,
}

impl<'a, T> TraqueurDeLimite<'a, T> where T: Messager, {
    pub fn new(messager: &T, max: usize) -> TraqueurDeLimite<T> {
        TraqueurDeLimite {
            messager,
            valeur: 0,
            max,
        }
    }

    pub fn set_valeur(&mut self, valeur: usize) {
        self.valeur = valeur;

        let pourcentage__du_maximum = self.valeur as f64/ self.max as f64; 

        if pourcentage__du_maximum >= 1.0 {
            self.messager.envoyer("Erreur : vous avez dépassé votre quota ! ");
        } else if pourcentage__du_maximum >= 0.9 {
            self.messager.envoyer("Attention, vous avez utilisé 90% de votre quota !");
        } else if pourcentage__du_maximum >= 0.75 {
            self.messager.envoyer("Attention, vous avez utilisé 75% de votre quota !");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MessagerMock {
        messages_envoyes: RefCell<Vec<String>>,
    }

    impl MessagerMock {
        fn new() -> MessagerMock {
            MessagerMock {
                messages_envoyes: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MessagerMock {
        fn envoyer(&self, msg: &str) {
            // self.messages_envoyes.borrow_mut().push(String::from(msg));
            let mut premier_emprunt = self.messages_envoyes.borrow_mut();
            let mut second_emprunt = self.messages_envoyes.borrow_mut();

            premier_emprunt.push(String::from(msg));
            second_emprunt.push(String::from(msg));
        }
    }

    #[test]
    fn envoi_d_un_message_d_avertissement_superieur_a__pourcent() {
        let messager_mock = MessagerMock::new();
        let mut traqueur = TraqueurDeLimite::new(&messager_mock, 100);
        traqueur.set_valeur(80);
        assert_eq!(messager_mock.messages_envoyes.borrow().len(), 1);
    }
}

// -------------------------- RUST BOOK - 15.6 --------------------------

fn boucles_de_reference(){
    println!("RUST BOOK - 15.6 Les boucles de références qui peuvent provoquer des fuites de mémoire ");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("compteur initial de a = {}", Rc::strong_count(&a));
    println!("prochain élément de a = {:?}", a.parcourir());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("compteur de a après création de b = {}", Rc::strong_count(&a));
    println!("compteur initial de b = {}", Rc::strong_count(&b));
    println!("prochain élément de b = {:?}", b.parcourir());

    if let Some(lien) = a.parcourir() {
        *lien.borrow_mut() = Rc::clone(&b); 
    }

    println!("compteur de b après avoir changé a = {}", Rc::strong_count(&b));
    println!("compteur de a après avoir changé a = {}", Rc::strong_count(&a));

    // Décommentez la ligne suivante pour constater que nous sommes dans 
    // une boucle de références, cela fera déborder la pile
    // println!("prochain élément de a = {:?}", a.parcourir());

    let feuille = Rc::new(Noeud {
        valeur: 3,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![]),
    });

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());

    let branche = Rc::new(Noeud {
        valeur: 5,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![Rc::clone(&feuille)]),
    });

    *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());

    println!(
        "feuille strong = {}, weak = {}", 
        Rc::strong_count(&feuille),    
        Rc::weak_count(&feuille),    
    );
    {
        let branche = Rc::new(Noeud {
            valeur: 5,
            parent: RefCell::new(Weak::new()),
            enfants: RefCell::new(vec![Rc::clone(&feuille)]),
        });

        *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

        println!(
            "branche strong = {}, weak = {}",
            Rc::strong_count(&branche),
            Rc::weak_count(&branche),
        );

        println!(
            "feuille strong = {}, weak = {}",
            Rc::strong_count(&feuille),
            Rc::weak_count(&feuille),
        );
    }

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());
    println!(
        "feuille strong = {}, weak = {}",
        Rc::strong_count(&feuille),
        Rc::weak_count(&feuille),
    );
}

impl List {
    fn parcourir(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Noeud {
    valeur: i32,
    parent: RefCell<Weak<Noeud>>,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}

