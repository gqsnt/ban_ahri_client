use iced::Color;

pub enum CustomColor {
    Red,
    Green,
    Blue,
}


impl CustomColor {
    pub fn to_color(&self) -> Color {
        match self {
            CustomColor::Red => Color::new(1.0, 0.0, 0.0, 1.0),
            CustomColor::Green => Color::new(0.0, 1.0, 0.0, 1.0),
            CustomColor::Blue => Color::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}