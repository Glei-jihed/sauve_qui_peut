# sauve_qui_peut

# feat: Initial project setup & basic client-server communication ( commit code : 8b54e52c995186bd6c47869e7ad8040ab1ac27ec )

- Création d'un workspace Cargo avec trois crates : client, shared et server.
- Implémentation des définitions communes des messages (RegisterTeam, RegisterTeamResult, SubscribePlayer, etc.) dans `shared/src/messages.rs`.
- Mise en place du module d'encodage personnalisé (base64, encodage de labyrinthes et de la vue radar) dans `shared/src/encodings.rs`.
- Développement du client (`client/src/main.rs` et `client/src/game.rs`) pour établir une connexion TCP avec le serveur, envoyer un message d'enregistrement d'équipe (RegisterTeam) en enveloppant le payload dans une clé et en préfixant le message par sa taille encodée en little-endian.
- Implémentation d'un serveur minimal (`server/src/main.rs`) capable de lire le message (taille + JSON), de désérialiser le message enveloppé et de renvoyer une réponse fictive (RegisterTeamResult avec un token).
- Validation de la communication de base entre client et serveur (connexion établie, message envoyé, réponse reçue avec un token).

Ce commit établit les bases de la communication et de la structuration du projet pour la suite du développement.


# feat(gui): Ajout de l'interface de création d'équipe

- Implémentation de l'application GUI TeamRegistrationApp avec eframe/egui.
- Mise en place d'un formulaire permettant de saisir le nom de l'équipe et la liste des membres.
- Affichage en console des informations saisies lors du clic sur "Enregistrer l'équipe" pour valider le fonctionnement de l'interface.
  
Prochaine étape :
- Intégrer la communication réseau en connectant l'interface à la logique du client via des canaux,
  afin d'envoyer les données de création d'équipe (team_name et team_members) au serveur de référence.


# feat(gui-styling): Amélioration de l'interface de création d'équipe avec mise en page en deux colonnes et animations dynamiques

- Mise en place d'une disposition en deux colonnes fixes :
  - Colonne gauche : formulaire de saisie pour le nom de l'équipe et les membres, avec des champs de taille agrandie et un fond animé qui passe d'un blanc cassé à une teinte rouge lors du focus.
  - Colonne droite : affichage d'une grande image (team-with-title.png) et d'un texte animé ("Sauve qui peut... en sortirez-vous vivants ?") dont la couleur et la taille changent dynamiquement, le tout contenu dans une zone dont la largeur est limitée par celle de l'image.
- Ajout d'effets dynamiques sur les champs et le bouton pour renforcer l'aspect immersif et moderne de l'interface.

Next steps:
- Intégrer la logique de jeu (déplacements, radar view, challenges) et la communication réseau complète pour l'inscription et la gestion du jeu.

# feat(player-registration): Implémentation de subscribe_player et intégration dans le thread réseau

- Ajout de la méthode subscribe_player dans GameClient (game.rs) pour envoyer une requête SubscribePlayer au serveur en utilisant le token d'inscription.
- Mise à jour du thread réseau dans main.rs afin d'appeler subscribe_player pour chaque membre de l'équipe après l'enregistrement.
- Les réponses du serveur pour SubscribePlayer sont désormais affichées dans la console.
  
Prochaine étape : traiter et afficher la réponse SubscribePlayer dans l'interface, et passer à l'intégration de la vue de jeu (RadarView, etc.).

# feat(game-view): Ajout d'une interface graphique pour la vue radar du labyrinthe

- Ajout du module `game_gui.rs` qui affiche une grille simulant le labyrinthe (RadarView).
  - La grille est centrée horizontalement.
  - Chaque cellule a une taille fixe avec un espacement, et les murs sont affichés en gris foncé.
  - La position du joueur est indiquée en vert.
- Mise à jour de `main.rs` pour lancer l'application GameView.
  
Next steps:
- Intégrer la réception réelle des données RadarView depuis le serveur et mettre à jour dynamiquement la grille.
- Ajouter des contrôles pour déplacer le joueur.

# feat(gui-styling): Amélioration interface création d’équipe


# feat(ui): feat(game-info-display): Affichage des infos d'inscription dans le mode jeu
- Modifié AppState dans app.rs pour inclure token, team_name et team_members dans la variante Game.
- Ajout d’un TopBottomPanel dans MainApp pour afficher le nom de l’équipe, les membres et le token au-dessus de la vue de jeu.
- Mise à jour du traitement des messages réseau pour transmettre ces informations.

- Ajout d'un champ rotation_angle dans MainApp pour gérer l'animation fluide du «cube».
- Incrémentation de rotation_angle à chaque frame pour un mouvement plus régulier.
- Réorganisation de l'interface en deux colonnes (informations + bouton à gauche, cube texturé tournant à droite).
- Utilisation de random_maze.png comme texture appliquée sur le carré rotatif.
- Conservation du panneau supérieur d'infos dans l'état Game avec la vue RadarView en dessous.


# feat(radar-view): add dynamic RadarView simulation

- Ajout d'un champ `last_update` dans GameView pour suivre le temps écoulé.
- Mise à jour de la grille de RadarView toutes les 2 secondes en modifiant aléatoirement les cellules (simulation de mise à jour).
- Intégration de la dépendance `rand` pour générer des valeurs aléatoires.
- Affichage de la grille 7x7 avec la cellule centrale en vert.

# feat(ui): enhance GameSetup screen with two-column layout and dynamic rotating cube

- Implemented a two-column layout in the GameSetup screen:
  - Left column: displays team details (team name, members, token) with classic black/gray/white styling and clear separators.
  - Right column: shows a dynamic rotating cube textured with "random_maze.png", simulating a 3D effect.
- Added a large "Lancer la partie" button below the team info that transitions the app to the game view.
- Improved mesh creation and rotation using a cumulative rotation angle for smoother animation.
- Maintained persistent team info display in the Game state.


