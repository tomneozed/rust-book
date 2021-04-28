
pub fn poo() {
    println!("RUST BOOK - 17. Les fonctionnalités orientées objet de Rust");

    // Cours
    // les_caracteristiques_poo();
    // les_traits_et_types_differents();
    patron_de_conception_oriente_objet();
    
    // Exos

}

// -------------------------- RUST BOOK - 17.1 --------------------------

fn les_caracteristiques_poo(){

}

pub struct CollectionMoyenne {
    liste: Vec<i32>,
    moyenne: f64,
}

impl CollectionMoyenne {
    pub fn ajouter(&mut self, valeur: i32) {
        self.liste.push(valeur);
        self.mettre_a_jour_moyenne();
    }

    pub fn retirer(&mut self) -> Option<i32> {
        let resultat = self.liste.pop();
        match resultat {
            Some(valeur) => {
                self.mettre_a_jour_moyenne();
                Some(valeur)
            }
            None => None,
        }
    }

    pub fn moyenne(&self) -> f64 {
        self.moyenne
    }

    fn mettre_a_jour_moyenne(&mut self) {
        let total: i32 = self.liste.iter().sum();
        self.moyenne = total as f64 / self.liste.len() as f64;
    }
}

// -------------------------- RUST BOOK - 17.2 --------------------------

fn les_traits_et_types_differents() {

    let ecran = Ecran {
        composants: vec![
            Box::new(ListeDeroulante {
                largeur: 75,
                hauteur: 10,
                options: vec![
                    String::from("Oui"),
                    String::from("Peut-être"),
                    String::from("Non"),
                ],
            }),
            Box::new(Bouton {
                largeur: 50,
                hauteur: 10,
                libelle: String::from("OK"),
            }),
        ],
    };

    ecran.executer();

    // let ecran = Ecran {
    //     composants: vec![Box::new(String::from("Salut"))],
    // };

    // ecran.executer();
}

pub trait Affichable {
    fn afficher(&self);
}

pub struct Ecran {
    pub composants: Vec<Box<dyn Affichable>>,
}

impl Ecran  {
    pub fn executer(&self) {
        for composant in self.composants.iter() {
            composant.afficher();
        }
    }
}

// pub struct Ecran<T: Affichable> {
//     pub composants: Vec<T>,
// }

// impl<T> Ecran<T>
// where
//     T: Affichable,
// {
//     pub fn executer(&self) {
//         for composant in self.composants.iter() {
//             composant.afficher();
//         }
//     }
// }

pub struct Bouton {
    pub largeur: u32,
    pub hauteur: u32,
    pub libelle: String,
}

impl Affichable for Bouton {
    fn afficher(&self) {
        // code servant vraiment à afficher un bouton
    }
}

struct ListeDeroulante {
    largeur: u32,
    hauteur: u32,
    options: Vec<String>,
}

impl Affichable for ListeDeroulante {
    fn afficher(&self) {
        // code servant vraiment à afficher une liste déroulante
    }
}

// -------------------------- RUST BOOK - 17.3 --------------------------

fn patron_de_conception_oriente_objet() {
    let mut billet = Billet::new();

    billet.ajouter_texte("J'ai mangé une salade ajd");
    // assert_eq!("", billet.contenu());

    let billet = billet.demander_relecture();
    // assert_eq!("", billet.contenu());

    let billet = billet.approuver();
    assert_eq!("J'ai mangé une salade ajd", billet.contenu());
}

pub struct Billet {
    // etat: Option<Box<dyn Etat>>,
    contenu: String,
}

impl Billet {
    // pub fn new() -> Billet {
    //     Billet {
    //         etat: Some(Box::new(Brouillon {})),
    //         contenu: String::new(),
    //     }
    // }

    pub fn new() -> BrouillonDeBillet {
        BrouillonDeBillet {
            contenu: String::new(),
        }
    }

    pub fn contenu(&self) -> &str {
        &self.contenu
    }

    // pub fn ajouter_texte(&mut self, texte: &str) {
    //     self.contenu.push_str(texte);
    // }

    // pub fn contenu<'a>(&self) -> &str {
    //     self.etat.as_ref().unwrap().contenu(self)
    // }

    // pub fn demander_relecture(&mut self) {
    //     if let Some(s) = self.etat.take() {
    //         self.etat = Some(s.demander_relecture())
    //     }
    // }

    // pub fn approuver(&mut self) {
    //     if let Some(s) = self.etat.take() {
    //         self.etat = Some(s.approuver())
    //     }
    // }
}

trait Etat {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
    fn approuver(self: Box<Self>) -> Box<dyn Etat>;
    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        ""
    }
}

struct Brouillon {}

impl Etat for Brouillon {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture {})
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}

struct EnRelecture {}

impl Etat for EnRelecture {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(Publier {})
    }
}

struct Publier {}

impl Etat for Publier {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        &billet.contenu
    }
}

pub struct BrouillonDeBillet {
    contenu: String,
}

impl BrouillonDeBillet {
    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn demander_relecture(self) -> BilletEnRelecture {
        BilletEnRelecture {
            contenu: self.contenu, 
        }
    }
}

pub struct BilletEnRelecture {
    contenu: String,
}

impl BilletEnRelecture {
    pub fn approuver(self) -> Billet {
        Billet {
            contenu: self.contenu
        }
    }
}

