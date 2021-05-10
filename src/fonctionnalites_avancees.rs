use std::slice;
use std::ops::Add;
use std::fmt;
use std::io::Error;
// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

use fmt::{Formatter, Result};

pub fn fonctionnalites_avancees() {
    println!("RUST BOOK - 18. Les motifs et le filtrage par motif");

    // Cours

    rust_unsafe();
    traits_avances();
    types_avances();
    fonctions_fermetures_avancees();
    les_macros();
}

// -------------------------- RUST BOOK - 19.1 --------------------------

pub fn rust_unsafe() {

    // 1er super-pouvoir : Déréférencer un pointeur brut

    let mut nombre = 5;

    // pointeur brut immuable
    let r1 = &nombre as *const i32;

    // pointeur brut mutable
    let r2 = &mut nombre as *mut i32;

    let addresse = 0x012345usize;
    // let r = addresse as *const i32;

    // 2eme super-pouvoir : Faire appel à une fonction ou une méthode non sécurisée
    
    unsafe {
        println!("r1 vaut {}", *r1);
        println!("r2 vaut {}", *r2);
        dangereux();
    }
    // dangereux();

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let r = addresse as *mut i32;

    // let slice = &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }

    unsafe {
        println!("la valeur absolue de -3 selon le langage C : {}", abs(-3));
    }

    // 3eme super-pouvoir : Lire ou modifier une variable statique mutable

    static HELLO_WORLD: &str = "Hello world";

    println!("Cela vaut : {}", HELLO_WORLD);

    ajouter_au_compteur(3);

    unsafe {
        println!("COMPTEUR : {}", COMPTEUR);
    }
}

unsafe fn dangereux() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COMPTEUR: u32 = 0;

fn ajouter_au_compteur(valeur: u32) {
    unsafe {
        COMPTEUR += valeur;
    }
}

// 4eme super-pouvoir : Implémenter un trait non sécurisé

unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

// -------------------------- RUST BOOK - 19.2 --------------------------

pub fn traits_avances() {

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    let une_personne = Humain;
    une_personne.voler();
    Pilote::voler(&une_personne);
    Magicien::voler(&une_personne);

    println!("Un bébé chien s'appelle un {}", Chien::nom_bebe());
    // println!("Un bébé chien s'appelle un {}", Animal::nom_bebe());
    println!("Un bébé chien s'appelle un {}", <Chien as Animal>::nom_bebe());

    let w = Enveloppe(vec![String::from("hello"), String::from("world !")]);
    println!("w = {}", w);
}

//  ******************************************************************************
//  *                                                                            *
//  * <Type as Trait>::function(destinataire_si_methode, argument_suivant, ...); *
//  *                                                                            *
//  ******************************************************************************

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// impl Iterator for Compteur {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Milimetres(u32);
struct Metres(u32);

impl Add<Metres> for Milimetres {
    type Output = Milimetres;

    fn add(self, other: Metres) -> Milimetres {
        Milimetres(self.0 + (other.0 * 1000))
    }
}

trait Pilote {
    fn voler(&self);
}

trait Magicien {
    fn voler(&self);
}

struct Humain;
impl Pilote for Humain {
    fn voler(&self) {
        println!("Ici le capitaine qui vous parle");
    }
}

impl Magicien for Humain {
    fn voler(&self) {
        println!("Décolage");
    }
}

impl Humain {
    fn voler(&self) {
        println!("Cui-cui");
    }
}

trait Animal {
    fn nom_bebe() -> String;
}

struct Chien;

impl Chien {
    fn nom_bebe() -> String {
        String::from("Spot")
    }
}

impl Animal for Chien {
    fn nom_bebe() -> String {
        String::from("chiot")
    }
}

trait OutlinePrint: fmt::Display {
    fn OutlinePrint(&self) {
        let valeur = self.to_string();
        let largeur = valeur.len();
        println!("{}", "*".repeat(largeur + 4));
        println!("*{}*", " ".repeat(largeur + 2));
        println!("* {} *", valeur);
        println!("*{}*", " ".repeat(largeur + 2));
        println!("{}", "*".repeat(largeur + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

struct Enveloppe(Vec<String>);

impl fmt::Display for Enveloppe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// -------------------------- RUST BOOK - 19.3 --------------------------

pub fn types_avances() {
    type Kilometres = i32;

    let x: i32 = 5;
    let y: Kilometres = 5;

    println!("x + y = {}", x + y);
}

// -------------------------- RUST BOOK - 19.4 --------------------------

pub fn fonctions_fermetures_avancees() {

    let reponse = le_faire_deux_fois(ajouter_un, 5);
    println!("La réponse est : {}", reponse); 

    let liste_de_nombres = vec![1, 2, 3];
    let liste_de_chaines: Vec<String> = liste_de_nombres.iter().map(|i| i.to_string()).collect();

    let liste_de_chaines: Vec<String> = liste_de_nombres.iter().map(ToString::to_string).collect();

    let liste_de_statuts: Vec<Statut> = (0u32..20).map(Statut::Valeur).collect();
}

fn ajouter_un(x: i32) -> i32 {
    x + 1
}

fn le_faire_deux_fois(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Statut {
    Valeur(u32),
    Stop,
}

fn retourne_une_fermeture() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// -------------------------- RUST BOOK - 19.5 --------------------------

pub fn les_macros() {

    let v = vec![1, 2, 3];

    // Pancakes::hello_macro();

}

#[macro_export]
macro_rules! vec {
    ( $( $x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello macro, mon nom est pancakes !");
//     }
// }

