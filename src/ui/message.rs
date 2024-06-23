use iced_box::icon::LoadingResult;

use crate::AppResult;
use crate::ui::widget::gif;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(LoadingResult),
    BanAhri,
    AhriBanned(AppResult<()>),
    StopShowAhriGif,
    RiotPathChanged(String),
    GifLoaded(Result<gif::Frames, gif::Error>),
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


