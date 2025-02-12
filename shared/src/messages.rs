use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeam {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RegisterTeamResult {
    Ok { expected_players: u8, registration_token: String },
    Err(String),
}

// On renomme le champ en snake_case et on le renomme pour la compatibilité JSON si nécessaire.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeamResultWrapper {
    #[serde(rename = "RegisterTeamResult")]
    pub register_team_result: RegisterTeamResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribePlayer {
    pub name: String,
    pub registration_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SubscribePlayerResult {
    Ok,
    Err(String),
}

/// Le RadarView contient la vue du joueur, encodée en base64.
#[derive(Serialize, Deserialize, Debug)]
pub struct RadarView(pub String);

/// Enum pour les indices envoyés par le serveur.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Hint {
    RelativeCompass { angle: f32 },
    GridSize { columns: u32, rows: u32 },
    Secret(u64),
    SOSHelper,
}

/// Les actions possibles du joueur.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Action {
    MoveTo(RelativeDirection),
    SolveChallenge { answer: String },
}

/// Direction relative pour le mouvement.
#[derive(Serialize, Deserialize, Debug)]
pub enum RelativeDirection {
    Front,
    Right,
    Back,
    Left,
}

/// Erreurs possibles lors d'une action.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionError {
    CannotPassThroughWall,
    CannotPassThroughOpponent,
    NoRunningChallenge,
    SolveChallengeFirst,
    InvalidChallengeSolution,
}

/// Les challenges envoyés par le serveur.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Challenge {
    SecretSumModulo(u64),
    SOS,
}
