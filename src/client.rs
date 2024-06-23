use std::path::PathBuf;
use std::sync::Arc;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::Client;
use tokio::sync::{mpsc, Mutex};

use crate::{AHRI_ID, AppError, AppResult};
use crate::client::champ_select_session::ChampSelectSession;

pub mod champ_select_session;

pub fn check_riot_path(riot_path: String) -> bool {
    PathBuf::from(riot_path).join("League of Legends").exists()
}


pub async fn ban_ahri(riot_path: String) -> AppResult<()> {
    let start = std::time::Instant::now();
    let client = LolClient::new(riot_path)?;
    let champ_select_session = client.get_champ_select_session().await?;
    if champ_select_session.timer.phase != "BAN_PICK" {
        return Err(AppError::RiotClientError("Not in Ban Phase".to_string()));
    }
    if champ_select_session.bans.my_team_bans.iter().any(|&champion_id| champion_id == AHRI_ID)
        || champ_select_session.bans.their_team_bans.iter().any(|&champion_id| champion_id == AHRI_ID) {
        return Err(AppError::RiotClientError("Ahri already banned".to_string()));
    }
    for actions in &champ_select_session.actions {
        for action in actions {
            if action.actor_cell_id == champ_select_session.local_player_cell_id && action.type_field == "ban" {
                client.ban_champion(AHRI_ID, action.id).await?;
                println!("Ahri banned in {:?}ms", start.elapsed().as_millis());
                return Ok(());
            }
        }
    }
    Err(AppError::RiotClientError("No ban action found".to_string()))
}


pub async fn stop_ban_ahri_thread(sender: Arc<Mutex<mpsc::Sender<bool>>>) -> AppResult<()> {
    let sender = sender.lock().await;
    sender.send(true).await.unwrap();
    Ok(())
}


#[derive(Clone, Default, Debug)]
pub struct LolClient {
    pub client: Client,
    pub port: String,
}


impl LolClient {
    pub fn new(riot_path: String) -> AppResult<Self> {
        let lol_path = PathBuf::from(riot_path).join("League of Legends");
        if !lol_path.exists() {
            return Err(AppError::IoError("League of Legends not found".to_string()));
        }
        let lockfile_path = lol_path
            .join("lockfile");
        if lockfile_path.exists() {
            let lockfile = std::fs::read_to_string(lockfile_path)?;
            let lockfile_parts: Vec<&str> = lockfile.split(':').collect();

            let port = lockfile_parts[2].to_string();
            let password = lockfile_parts[3];

            let mut headers = reqwest::header::HeaderMap::new();
            let auth = BASE64_STANDARD.encode(format!("riot:{}", password).as_str());
            headers.insert("Authorization", format!("Basic {}", auth).parse().unwrap());

            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .default_headers(headers)
                .build()
                .unwrap();
            Ok(Self { port, client })
        } else {
            Err(AppError::IoError("lockfile not found".to_string()))
        }
    }

    pub fn get_url(&self, endpoint: &str) -> String {
        format!("https://127.0.0.1:{}{}", &self.port, endpoint)
    }


    pub async fn ban_champion(&self, champion_id: i32, action_id: i64) -> AppResult<()> {
        let url = self.get_url(format!("/lol-champ-select/v1/session/actions/{}", action_id).as_str());
        self.client.patch(url)
            .json(&serde_json::json!({
            "championId": champion_id,
            "completed": true,
        }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }


    pub async fn get_champ_select_session(&self) -> AppResult<ChampSelectSession> {
        let url = self.get_url("/lol-champ-select/v1/session");
        let response = self.client.get(&url).send().await?;
        response.json::<ChampSelectSession>().await.map_err(|_| AppError::RiotClientError("No Champ Select Session".to_string()))
    }

    pub async fn get_user_session(&self) -> AppResult<serde_json::Value> {
        let url = self.get_url("/lol-chat/v1/me");
        let response = self.client.get(&url).send().await?;
        response.json().await.map_err(|_| AppError::RiotClientError("No User Session".to_string()))
    }
}