// Importation du module gpio
mod Gpio;

fn main() {
    // Création d'un objet GpioPin pour le pin numéro 2, configuré en sortie
    let mut pin = Gpio::GpioPin::new(2, Gpio::Direction::Output);

    // Configuration du pin en sortie
    pin.configure_as_output();

    // Écriture de la valeur 'true' (HIGH) sur le pin pour l'activer
    pin.write(true);

    // Reconfiguration du pin en entrée pour pouvoir lire son état
    pin.configure_as_input();

    // Lecture de l'état du pin et affichage de la valeur lue
    let state = pin.read();
    println!("L'état du pin est: {}", state);
}
