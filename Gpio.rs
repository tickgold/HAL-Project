// Définition d'une structure pour représenter un pin GPIO
// Cette structure contient le numéro du pin et sa direction (entrée ou sortie)
pub struct GpioPin {
    pub pin_number: u8,       // Numéro du pin (par exemple, 2 pour le pin D2)
    pub direction: Direction, // Direction du pin (entrée ou sortie)
}

// Enumération pour représenter les directions possibles d'un pin GPIO
pub enum Direction {
    Input,  // Le pin est configuré en entrée
    Output, // Le pin est configuré en sortie
}

// Implémentation des méthodes pour la structure GpioPin
impl GpioPin {
    // Fonction pour créer un nouveau pin GPIO avec un numéro de pin et une direction spécifiée
    pub fn new(pin_number: u8, direction: Direction) -> Self {
        GpioPin { pin_number, direction }
    }

    // Méthode pour configurer le pin en entrée
    pub fn configure_as_input(&mut self) {
        // Ici, on ajouterait la logique spécifique pour configurer le pin en entrée
        // Par exemple, utiliser les registres du microcontrôleur pour définir le mode du pin
        self.direction = Direction::Input; // Met à jour la direction dans la structure
        println!("Pin {} configuré en entrée.", self.pin_number);
    }

    // Méthode pour configurer le pin en sortie
    pub fn configure_as_output(&mut self) {
        // Ici, on ajouterait la logique spécifique pour configurer le pin en sortie
        // Cela inclut potentiellement la configuration du registre de direction pour le microcontrôleur
        self.direction = Direction::Output; // Met à jour la direction dans la structure
        println!("Pin {} configuré en sortie.", self.pin_number);
    }

    // Méthode pour lire l'état du pin (retourne vrai pour HIGH et faux pour LOW)
    pub fn read(&self) -> bool {
        // Cette méthode simule la lecture d'un pin GPIO
        // Dans une implémentation réelle, il faudrait accéder au registre d'entrée du microcontrôleur
        println!("Lecture de l'état du pin {}.", self.pin_number);
        false // Pour l'exemple, renvoie toujours faux (LOW)
    }

    // Méthode pour écrire une valeur sur le pin (true pour HIGH et false pour LOW)
    pub fn write(&self, value: bool) {
        // Ici, on ajouterait la logique pour écrire une valeur sur le pin
        // Cela pourrait inclure la modification du registre de sortie du microcontrôleur
        if let Direction::Output = self.direction {
            // Vérifie d'abord que le pin est configuré en sortie avant d'écrire
            println!("Écriture de la valeur {} sur le pin {}.", value, self.pin_number);
        } else {
            println!("Impossible d'écrire sur le pin {} car il est configuré en entrée.", self.pin_number);
        }
    }
}
