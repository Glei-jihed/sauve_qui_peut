use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeam {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeamResultOk {
    pub expected_players: u8,
    pub registration_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RegisterTeamResult {
    OkVariant { #[serde(rename = "Ok")] ok: RegisterTeamResultOk },
    ErrVariant { #[serde(rename = "Err")] err: String },
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RadarView(pub String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Hint {
    RelativeCompass { angle: f32 },
    GridSize { columns: u32, rows: u32 },
    Secret(u64),
    SOSHelper,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Action {
    MoveTo(RelativeDirection),
    SolveChallenge { answer: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RelativeDirection {
    Front,
    Right,
    Back,
    Left,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionError {
    CannotPassThroughWall,
    CannotPassThroughOpponent,
    NoRunningChallenge,
    SolveChallengeFirst,
    InvalidChallengeSolution,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Challenge {
    SecretSumModulo(u64),
    SOS,
}
