pub mod data;

use iced::{
    application,
    widget::{button, container, text},
    Background, Color,
};

use self::data::ThemeData;

#[derive(Default)]
pub struct Theme {
    pub data: ThemeData,
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::from(self.data.theme.background),
            text_color: Color::from(self.data.theme.text),
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: 2.0,
            background: Some(Background::Color(Color::from(
                self.data.theme.button.active,
            ))),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: 2.0,
            background: Some(Background::Color(Color::from(
                self.data.theme.button.hovered,
            ))),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: 2.0,
            background: Some(Background::Color(Color::from(
                self.data.theme.button.pressed,
            ))),
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: 2.0,
            background: Some(Background::Color(Color::from(
                self.data.theme.button.disabled,
            ))),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum Container {
    #[default]
    Transparent,
    Background,
    BackgroundContrast1,
    BackgroundContrast2,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: match style {
                Container::Transparent => None,
                Container::Background => {
                    Some(Background::Color(Color::from(self.data.theme.background)))
                }
                Container::BackgroundContrast1 => Some(Background::Color(Color::from(
                    self.data.theme.background_contrast1,
                ))),
                Container::BackgroundContrast2 => Some(Background::Color(Color::from(
                    self.data.theme.background_contrast2,
                ))),
            },
            ..Default::default()
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        Default::default()
    }
}
