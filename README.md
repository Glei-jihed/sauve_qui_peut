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
