// Modules
mod erreurs;
mod collections;
mod generiques;
mod tests;
mod langages_fonctionnels;
mod pointeurs;

// Imports
use std::{borrow::BorrowMut, collections::HashMap, io, ops::Div};

use collections::collections;
use erreurs::gestion_des_erreurs;
use generiques::generiques;
use tests::tests;
use langages_fonctionnels::langages_fonctionnels;
use pointeurs::pointeurs_intelligents;

use rust_book::CouleurPrimaire;
use rust_book::mixer;


fn main() {
    //collections();
    //gestion_des_erreurs();
    // generiques();
    // tests();
    // langages_fonctionnels();
    pointeurs_intelligents();

    let rouge = CouleurPrimaire::Rouge;
    let jaune = CouleurPrimaire::Jaune;
    mixer(rouge, jaune);
}
