use std::sync::Arc;

use iced::{Application, executor, Length};
use iced::{Command, Element, Theme};
use iced::widget::{Column, Row, text, text_input};
use iced::widget::container;
use iced_box::icon::material::{load_material_font, Material};
use tokio::sync::{mpsc, Mutex};

use crate::client::{check_riot_path, start_ban_ahri_thread, stop_ban_ahri_thread};
use crate::DEFAULT_THREAD_WAIT_TIME;
use crate::ui::message::Message;
use crate::ui::widget::{custom_button, icons_builder};
use crate::ui::widget::custom_button::custom_button;

pub struct MainApp {
    is_banning_ahri: bool,
    is_path_valid: bool,
    ban_ahri_sender: Option<Arc<Mutex<mpsc::Sender<bool>>>>,
    riot_path: String,
    thread_wait_time: u64,

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
            ban_ahri_sender: None,
            is_banning_ahri: false,
            thread_wait_time: DEFAULT_THREAD_WAIT_TIME,
            is_path_valid,
            riot_path,
        }, Command::batch(vec![
            load_material_font().map(Message::FontLoaded),
        ]))
    }

    fn title(&self) -> String {
        "Ban Ahri Client".to_string()
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartBanAhri => {
                if self.is_path_valid {
                    Command::perform(start_ban_ahri_thread(self.riot_path.clone(), self.thread_wait_time), Message::BanAhriStarted)
                } else {
                    Command::none()
                }
            }
            Message::StopBanAhri => {
                if let Some(sender) = &self.ban_ahri_sender {
                    Command::perform(stop_ban_ahri_thread(sender.clone()), Message::BanAhriStopped)
                } else {
                    Command::none()
                }
            }
            Message::BanAhriStarted(sender) => {
                self.is_banning_ahri = true;
                self.ban_ahri_sender = Some(sender.unwrap());
                Command::none()
            }
            Message::BanAhriStopped(_) => {
                self.is_banning_ahri = false;
                self.ban_ahri_sender = None;
                Command::none()
            }
            Message::RiotPathChanged(path) => {
                self.riot_path = path;
                self.is_path_valid = check_riot_path(self.riot_path.clone());
                Command::none()
            }
            Message::ThreadWaitTimeChanged(time) => {
                self.thread_wait_time = time;
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
                .push(Row::new()
                    .push(text("Thread Wait Time:"))
                    .push(
                        text_input("Thread Wait Time", self.thread_wait_time.to_string().as_ref())
                            .on_input(|value| {
                                Message::ThreadWaitTimeChanged(value.parse().unwrap_or(DEFAULT_THREAD_WAIT_TIME))
                            })
                    )
                    .spacing(10))
                .push(
                    if self.is_banning_ahri {
                        custom_button("Stop Ban Ahri")
                            .style(custom_button::danger)
                            .padding([30, 110])
                            .on_press(Message::StopBanAhri)
                            .width(Length::Fill)
                            .height(Length::Fill)
                    } else {
                        custom_button("Start Ban Ahri")
                            .style(custom_button::primary)
                            .padding([30, 110])
                            .on_press(Message::StartBanAhri)
                            .width(Length::Fill)
                            .height(Length::Fill)
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

