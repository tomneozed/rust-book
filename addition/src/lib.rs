// #[cfg(test)] -> ce code ne sera compilé et exécuté qu'avec 'cargo test' (et non pas 'cargo build') -> gain de temps
use langagesFonctionnels::langagesFonctionnels;
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn un_autre() {
    //     panic!("Fais échouer le test");
    // }

    // Vérifier les résultats avec la macro assert! -> bool

    #[derive(Debug)]
    struct Rectangle {
        largeur: u32,
        hauteur: u32,
    }

    impl Rectangle {
        fn peut_contenir(&self, other: &Rectangle) -> bool {
            self.largeur > other.largeur && self.hauteur > other.hauteur 
        }
    }

    #[test]
    fn un_grand_peut_contenir_un_petit() {
        let le_grand = Rectangle { largeur: 10, hauteur: 20 };
        let le_petit = Rectangle { largeur: 9, hauteur: 15 };

        assert!(le_grand.peut_contenir(&le_petit));
    }

    #[test]
    fn un_petit_ne_peut_pas_contenir_un_grand() {
        let le_grand = Rectangle { largeur: 10, hauteur: 20 };
        let le_petit = Rectangle { largeur: 9, hauteur: 15 };

        assert!(!le_petit.peut_contenir(&le_grand));
    }

    // Tester l'égalité avec les macros assert_eq! et assert_ne!

    pub fn ajouter_deux(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn cela_ajoute_deux() {
        assert_eq!(ajouter_deux(2), 4);
        assert_ne!(ajouter_deux(2), 5);
    }

    // Ajouter des messages d'échec personnalisés

    pub fn acceuil(nom: &str) -> String {
        format!("Salut, {}", nom)
    }

    #[test]
    fn acceuil_contient_le_nom() {
        let resultat = acceuil("Carole");
        assert!(
            resultat.contains("Carole"), 
            "Le message d'acceuil ne contient pas le nom, il vaut `{}`",
            resultat
        );
    }

    // Vérifier le fonctionnement des paniques avec should_panic

    pub struct Supposition {
        valeur: i32,
    }

    impl Supposition {
        pub fn new(valeur: i32) -> Supposition {
            if valeur < 1 {
                panic!("La supposition doit etre > ou = 1, nous avons : {}", valeur);
            } else if  valeur > 100 {
                panic!("La supposition doit etre < ou = 100, nous avons : {}", valeur);
            }
            Supposition { valeur }
        }
    }

    #[test]
    #[should_panic]
    fn en_dehors_des_limites(){
        Supposition::new(200);
    }

    #[test]
    #[should_panic(expected = "La supposition doit etre < ou = 100")]
    fn plus_grand_que_100(){
        Supposition::new(200);
    }

    // Utiliser Result<T, E> dans les tests -> permet d'utiliser '?'

    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from(" 2 + 2 != 4"))
        }
    }

    // 11.2 - Gérer l'exécution des tests

    // Afficher la sortie de la fonction

    fn affiche_et_retourne_10(a: i32) -> i32 {
        println!("J'ai obtenu la valeur de {}", a);
        10
    }

    #[test]
    fn ce_test_reussit(){
        let valeur = affiche_et_retourne_10(4);
        assert_eq!(valeur, 10);
    }

    #[test]
    #[ignore]
    fn ce_test_echoue(){
        let valeur = affiche_et_retourne_10(8);
        assert_eq!(valeur, 5);
    }
    
    // Ignorer certains tests sauf s'ils sont demandés explicitement

    #[test]
    #[ignore]
    fn test_ignore() {
        /* blabla */
    }
}

pub fn ajouter_deux(a: i32) -> i32 {
    a + 2
}


