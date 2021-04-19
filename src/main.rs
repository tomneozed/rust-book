// Modules
mod erreurs;
mod collections;
mod generiques;
mod tests;
mod langagesFonctionnels;

// Imports
use std::{borrow::BorrowMut, collections::HashMap, io, ops::Div};

use collections::collections;
use erreurs::gestion_des_erreurs;
use generiques::generiques;
use tests::tests;
use langagesFonctionnels::langagesFonctionnels;


fn main() {
    //collections();
    //gestion_des_erreurs();
    // generiques();
    tests();
    langagesFonctionnels();
}
