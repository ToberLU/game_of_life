# Game of Life (Rust + Raylib)

Simulateur du Jeu de la Vie de Conway, développé en Rust avec Raylib. Projet d'apprentissage des idiomes Rust.

## Fonctionnalités actuelles
- Fenêtre Raylib 1000x800
- Logging structuré avec `tracing` (spans, sortie fichier quotidienne)
- Architecture modulaire (`grid`, `simulation`, `render`)

## Prérequis
- Rust (édition 2024) et Cargo

## Lancement
```bash
cargo run
```

## Structure
```
src/
├── main.rs       # Entrée, init tracing
├── grid.rs       # Logique grille 2D
├── simulation.rs # Règles de simulation
└── render.rs     # Rendu Raylib
```

## Progression
Voir [TODO.md](TODO.md) pour l'avancement détaillé.
