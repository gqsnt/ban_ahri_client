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

pub async fn start_client(riot_path: String) -> AppResult<(LolClient, String)> {
    let client = LolClient::new(riot_path)?;
    let user_session = client.get_user_session().await?;
    user_session["puuid"].as_str()
        .map(|puuid| (client, puuid.to_string()))
        .ok_or(AppError::RiotClientError("Client Not Started".to_string()))
}

pub async fn start_ban_ahri_thread(riot_path: String, thread_wait_time:u64) -> AppResult<Arc<Mutex<mpsc::Sender<bool>>>> {
    let (tx, mut rx) = mpsc::channel(1);
    let arc_mutex_tx = Arc::new(Mutex::new(tx));
    tokio::spawn(async move {
        let mut riot_client: Option<LolClient> = None;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(thread_wait_time)).await;

            // Check if we should stop the thread(App message)
            if rx.try_recv().is_ok() {
                break;
            }

            // Initialize Riot client if not already done
            if riot_client.is_none() {
                match start_client(riot_path.clone()).await {
                    Ok((client, _)) => riot_client = Some(client),
                    Err(_) => continue,
                }
            }

            let client = riot_client.as_ref().expect("Client should be initialized");
            // reset if not connected to the client
            if client.get_user_session().await.is_err() {
                riot_client = None;
                continue;
            }

            let start = std::time::Instant::now();
            // check if we are in champ select
            match client.get_champ_select_session().await {
                Ok(champ_select_session) => {
                    // Check if it's the ban phase && if Ahri is not already banned
                    if champ_select_session.timer.phase != "BAN_PICK"
                        || champ_select_session.bans.my_team_bans.iter().any(|&champion_id| champion_id == AHRI_ID)
                    {
                        continue;
                    }
                    // find the correct action and ban Ahri
                    for actions in &champ_select_session.actions {
                        for action in actions {
                            if action.actor_cell_id == champ_select_session.local_player_cell_id && action.type_field == "ban" {
                                if let Err(err) = client.ban_champion(AHRI_ID, action.id).await {
                                    println!("Failed to ban Ahri: {:?}", err);
                                }else{
                                    println!("Ahri banned in {:?}ms", start.elapsed().as_millis());
                                }
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    });
    Ok(arc_mutex_tx)
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