use std::path::PathBuf;

use iced::{Application, executor, Length};
use iced::{Command, Element, Theme};
use iced::widget::{Column, Row, text, text_input};
use iced::widget::container;
use iced_box::icon::material::{load_material_font, Material};

use crate::client::{ban_ahri, check_riot_path};
use crate::ui::message::Message;
use crate::ui::widget::{custom_button, gif, icons_builder};
use crate::ui::widget::custom_button::custom_button;
use crate::wait_n_millis;

pub struct MainApp {
    is_banning_ahri: bool,
    is_path_valid: bool,
    riot_path: String,
    show_ahri_gif: bool,
    frames: Option<gif::Frames>,
}


impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();


    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let riot_path = "C:\\Riot Games".to_string();
        let is_path_valid = check_riot_path(riot_path.clone());
        (Self {
            is_banning_ahri: false,
            is_path_valid,
            riot_path,
            show_ahri_gif: false,
            frames: None,
        }, Command::batch(vec![
            load_material_font().map(Message::FontLoaded),
            gif::Frames::load_from_path(PathBuf::from("assets").join("ahri_by.gif")).map(Message::GifLoaded),
        ]))
    }

    fn title(&self) -> String {
        "Ban Ahri Client".to_string()
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BanAhri => {
                if self.is_path_valid {

                    self.is_banning_ahri = true;
                    Command::perform(ban_ahri(self.riot_path.clone()), Message::AhriBanned)
                } else {
                    Command::none()
                }
            }
            Message::AhriBanned(result) => {
                self.is_banning_ahri = false;

                if let Err(err) = result {
                    println!("Failed to ban Ahri: {:?}", err);
                    Command::none()
                } else {
                    self.show_ahri_gif = true;
                    Command::perform(wait_n_millis(1400), |_| Message::StopShowAhriGif)
                }

            }
            Message::RiotPathChanged(path) => {
                self.riot_path = path;
                self.is_path_valid = check_riot_path(self.riot_path.clone());
                Command::none()
            }
            Message::GifLoaded(frames) => {
                self.frames = frames.ok();
                Command::none()
            }
            Message::StopShowAhriGif => {
                self.show_ahri_gif = false;
                Command::none()
            }
            _ => { Command::none() }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(Column::new()
            .push(Column::new()
                .push(
                    Row::new()
                        .push(text("Riot Path:"))
                        .push(
                            text_input("Riot Path", self.riot_path.as_ref())
                                .on_input(Message::RiotPathChanged)
                        )
                        .push(if self.is_path_valid {
                            icons_builder(Material::CheckCircle).size(20).build()
                        } else {
                            icons_builder(Material::Close).size(20).build()
                        })
                        .spacing(10)
                )
                .push(
                    if self.show_ahri_gif {
                        container(gif(self.frames.as_ref().unwrap()))
                            .center_x()
                            .center_y()
                            .width(Length::Fill)
                            .height(Length::Fill)
                    } else {
                        container(custom_button("Ban Ahri")
                            .style(custom_button::primary)
                            .padding([30, 125])
                            .on_press(Message::BanAhri)
                            .width(Length::Fill)
                            .height(Length::Fill))
                    }
                )
                .spacing(20)
                .width(Length::Fill)
                .height(Length::Fill))
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

