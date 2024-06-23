use iced::{Application, Settings, Size};

use ban_ahri_client::ui::application::MainApp;

fn main() -> iced::Result {
    MainApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(360.0, 175.0),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}