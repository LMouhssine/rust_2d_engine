fn main() {
    println!("Démarrage du moteur de jeu...");
    
    // Initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Création de la fenêtre
    let window = video_subsystem.window("Mon Jeu", 800, 600)
        .position_centered()
        .build()
        .unwrap();
        
    println!("Fenêtre créée avec succès!");
}