// Modules
mod erreurs;
mod collections;
mod generiques;
mod tests;

// Imports
use std::{borrow::BorrowMut, collections::HashMap, io, ops::Div};

use collections::collections;
use erreurs::gestion_des_erreurs;
use generiques::generiques;
use tests::tests;


fn main() {
    //collections();
    //gestion_des_erreurs();
    // generiques();
    tests();
}
