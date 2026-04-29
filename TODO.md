# TODO - Game of Life avec Raylib

## État d'avancement

### ✅ Niveau débutant - Terminé
- [x] 1. Créer la structure de modules (`grid`, `simulation`, `render`)
- [x] 2. Initialiser le projet avec Raylib, ouvrir une fenêtre vide
- [x] 3. Ajouter `tracing` avec spans dans `render`

### 🔄 En cours
- [ ] 4. Configurer `tracing-appender` pour extraire les logs dans `logs/` (fichier par jour, append si même jour)

### ⏳ À venir

#### Niveau débutant (suite)
- [ ] 5. Créer la structure de grille 2D pour le Game of Life
- [ ] 6. Implémenter le calcul des voisins vivants

#### Niveau intermédiaire
- [ ] 7. Ajouter les contrôles clavier (pause, reset, vitesse)
- [ ] 8. Gérer les motifs initiaux (glider, pulsar)
- [ ] 9. Implémenter un trait commun pour les règles de simulation

#### Niveau avancé
- [ ] 10. Refactoriser avec des iterators pour le calcul de voisins
- [ ] 11. Ajouter des animations de transition (naissance/mort)
- [ ] 12. Sauvegarder/charger des états de grille
