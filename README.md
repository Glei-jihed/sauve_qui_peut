# sauve_qui_peut

feat: Initial project setup & basic client-server communication ( commit code : 8b54e52c995186bd6c47869e7ad8040ab1ac27ec )

- Création d'un workspace Cargo avec trois crates : client, shared et server.
- Implémentation des définitions communes des messages (RegisterTeam, RegisterTeamResult, SubscribePlayer, etc.) dans `shared/src/messages.rs`.
- Mise en place du module d'encodage personnalisé (base64, encodage de labyrinthes et de la vue radar) dans `shared/src/encodings.rs`.
- Développement du client (`client/src/main.rs` et `client/src/game.rs`) pour établir une connexion TCP avec le serveur, envoyer un message d'enregistrement d'équipe (RegisterTeam) en enveloppant le payload dans une clé et en préfixant le message par sa taille encodée en little-endian.
- Implémentation d'un serveur minimal (`server/src/main.rs`) capable de lire le message (taille + JSON), de désérialiser le message enveloppé et de renvoyer une réponse fictive (RegisterTeamResult avec un token).
- Validation de la communication de base entre client et serveur (connexion établie, message envoyé, réponse reçue avec un token).

Ce commit établit les bases de la communication et de la structuration du projet pour la suite du développement.
