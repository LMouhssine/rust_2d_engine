use specs::prelude::*;
use specs_derive::Component;

// Composant pour les objets pouvant être rendus à l'écran
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub width: f32,
    pub height: f32,
    pub color: (f32, f32, f32), // Valeurs RGB comprises entre 0 et 1
}

// Composant représentant un joueur
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub speed: f32, // Vitesse du joueur
}

// Composant pour les objets qui peuvent entrer en collision
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collidable {
    pub radius: f32, // Rayon pour la détection de collision
}

// Composant pour l'émetteur de particules
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ParticleEmitter {
    pub rate: f32, // Taux d'émission des particules
    pub lifetime: f32, // Durée de vie des particules
    pub color: (f32, f32, f32), // Couleur des particules (RGB entre 0 et 1)
}

// Composant représentant la position d'un objet
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32, // Coordonnée X
    pub y: f32, // Coordonnée Y
}

// Composant représentant la vitesse d'un objet
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32, // Vitesse sur l'axe X
    pub y: f32, // Vitesse sur l'axe Y
}
