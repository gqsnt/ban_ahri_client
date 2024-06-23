use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampSelectSession {
    pub actions: Vec<Vec<Action>>,
    pub bans: Bans,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
    pub timer: Timer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "actorCellId")]
    pub actor_cell_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    pub completed: bool,
    pub id: i64,
    #[serde(rename = "isAllyAction")]
    pub is_ally_action: bool,
    #[serde(rename = "isInProgress")]
    pub is_in_progress: bool,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bans {
    #[serde(rename = "myTeamBans")]
    pub my_team_bans: Vec<i32>,
    #[serde(rename = "numBans")]
    pub num_bans: i64,
    #[serde(rename = "theirTeamBans")]
    pub their_team_bans: Vec<i32>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    #[serde(rename = "adjustedTimeLeftInPhase")]
    pub adjusted_time_left_in_phase: i64,
    #[serde(rename = "internalNowInEpochMs")]
    pub internal_now_in_epoch_ms: i64,
    #[serde(rename = "isInfinite")]
    pub is_infinite: bool,
    pub phase: String,
    #[serde(rename = "totalTimeInPhase")]
    pub total_time_in_phase: i64,
}