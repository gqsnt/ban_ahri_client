use crate::AppResult;
use crate::client::LolClient;

#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub lol_client: LolClient,
    pub ban_ahri: bool,
}

pub async fn init_connected_state(riot_path: String) -> AppResult<ConnectedState> {
    let client = LolClient::new(riot_path)?;
    Ok(ConnectedState {
        lol_client: client,
        ..Default::default()
    })
}

