use iced::{Background, Color, Gradient, Renderer, Theme};
use iced::gradient::Linear;
use iced::widget::{container, Container, text};
use iced_box::icon::material::{Material, material_font};

use crate::ui::message::Message;
use crate::ui::widget::custom_button::{custom_button, CustomButton};

pub mod circle;
pub mod custom_button;

pub const DEFAULT_TEXT_SIZE: u16 = 20;


pub trait CanScaleAlpha {
    fn scale_alpha(self, alpha: f32) -> Self;
}

impl CanScaleAlpha for Color {
    fn scale_alpha(self, factor: f32) -> Self {
        Self {
            a: self.a * factor,
            ..self
        }
    }
}


impl CanScaleAlpha for Background {
    fn scale_alpha(self, factor: f32) -> Self {
        match self {
            Self::Color(color) => Self::Color(color.scale_alpha(factor)),
            Self::Gradient(gradient) => {
                Self::Gradient(gradient.scale_alpha(factor))
            }
        }
    }
}


impl CanScaleAlpha for Gradient {
    fn scale_alpha(self, factor: f32) -> Self {
        match self {
            Gradient::Linear(linear) => {
                Gradient::Linear(linear.scale_alpha(factor))
            }
        }
    }
}

impl CanScaleAlpha for Linear {
    fn scale_alpha(mut self, factor: f32) -> Self {
        for stop in self.stops.iter_mut().flatten() {
            stop.color.a *= factor;
        }

        self
    }
}

pub struct IconBuilder {
    icon: Material,
    size: u16,
}

impl IconBuilder {
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> Container<'static, Message, Theme, Renderer> {
        container(
            text(self.icon).font(material_font()).size(self.size)
        ).center_x().center_y()
    }
}


pub fn icons_builder(icon: Material) -> IconBuilder {
    IconBuilder {
        icon,
        size: DEFAULT_TEXT_SIZE,
    }
}


pub fn button_padded(text: &str) -> CustomButton<Message> {
    custom_button(text).style(custom_button::secondary).padding([12, 24])
}