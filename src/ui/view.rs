use iced::Command;
use iced::widget::Container;

use crate::ui::message::Message;
use crate::ui::state::ConnectedState;

pub trait HasView {
    type State;
    type Message;
    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message>;
    fn view(connected_sate: &ConnectedState) -> Container<'_, Message>;
}