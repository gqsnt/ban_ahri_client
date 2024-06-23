use std::sync::Arc;

use iced_box::icon::LoadingResult;
use tokio::sync::{mpsc, Mutex};

use crate::AppResult;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(LoadingResult),
    StartBanAhri,
    BanAhriStarted(AppResult<Arc<Mutex<mpsc::Sender<bool>>>>),
    StopBanAhri,
    BanAhriStopped(AppResult<()>),
    RiotPathChanged(String),
    ThreadWaitTimeChanged(u64),
}



macro_rules! impl_from_for_message {
    ($source_type:ty => $enum_variant:ident) => {
        impl From<$source_type> for Message {
            fn from(value: $source_type) -> Self {
                Self::$enum_variant(value)
            }
        }
    };
}


