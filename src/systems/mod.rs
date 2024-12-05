// src/systems/mod.rs

// Déclaration des modules
pub mod movement; // Ajoutez ce module pour le rendre public
pub mod collision;
pub mod particle;

// Export des systèmes
pub use self::collision::CollisionSystem;
pub use self::particle::ParticleSystem;
pub use self::movement::MovementSystem; // Ajoutez cette ligne pour rendre MovementSystem accessible
