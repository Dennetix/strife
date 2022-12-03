pub mod data;

use iced::{
    application,
    widget::{
        button, container,
        rule::{self, FillMode},
        text,
    },
    Background, Color,
};
use iced_native::widget::scrollable;

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

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,

    /// selected, border radius
    TransparentHover(bool, Option<f32>),

    /// selected, border radius, border width
    TransparentBorder(bool, Option<f32>, f32),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = button::Appearance {
            text_color: Color::from(self.data.theme.button.text),
            border_radius: self.data.theme.button.border_radius,
            ..Default::default()
        };

        match style {
            Button::Primary => {
                appearance.background =
                    Some(Background::Color(Color::from(self.data.theme.primary)));
            }
            Button::Secondary => {
                appearance.background =
                    Some(Background::Color(Color::from(self.data.theme.secondary)));
            }
            Button::TransparentHover(selected, border_radius) if *selected => {
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.transparent_pressed,
                )));
            }
            Button::TransparentBorder(selected, border_radius, border_width) => {
                appearance.border_width = *border_width;
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                if *selected {
                    appearance.border_color = Color::from(self.data.theme.primary);
                }
            }
            _ => {}
        }

        appearance
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = button::Appearance {
            text_color: Color::from(self.data.theme.button.text),
            border_radius: self.data.theme.button.border_radius,
            ..Default::default()
        };

        match style {
            Button::Primary => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.primary_hovered,
                )));
            }
            Button::Secondary => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.secondary_hovered,
                )));
            }
            Button::TransparentHover(_, border_radius) => {
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.transparent_hover,
                )));
            }
            Button::TransparentBorder(selected, border_radius, border_width) => {
                appearance.border_width = *border_width;
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                if *selected {
                    appearance.border_color = Color::from(self.data.theme.primary);
                } else {
                    appearance.border_color = Color::from(self.data.theme.secondary);
                }
            }
        }

        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = button::Appearance {
            text_color: Color::from(self.data.theme.button.text),
            ..Default::default()
        };

        match style {
            Button::Primary => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.primary_pressed,
                )));
            }
            Button::Secondary => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.secondary_pressed,
                )));
            }
            Button::TransparentHover(_, border_radius) => {
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.button.transparent_pressed,
                )));
            }
            Button::TransparentBorder(_, border_radius, border_width) => {
                appearance.border_width = *border_width;
                if let Some(border_radius) = border_radius {
                    appearance.border_radius = *border_radius;
                }
                appearance.border_color = Color::from(self.data.theme.primary);
            }
        }

        appearance
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
    BackgroundStrong1,
    BackgroundStrong2,

    /// border radius
    BackgroundWeak(f32),

    /// color, border radius
    Color(Color, f32),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let mut appearance = container::Appearance::default();

        match style {
            Container::Background => {
                appearance.background =
                    Some(Background::Color(Color::from(self.data.theme.background)));
            }
            Container::BackgroundStrong1 => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.background_strong1,
                )));
            }
            Container::BackgroundStrong2 => {
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.background_strong2,
                )));
            }
            Container::BackgroundWeak(border_radius) => {
                appearance.border_radius = *border_radius;
                appearance.background = Some(Background::Color(Color::from(
                    self.data.theme.background_weak,
                )));
            }
            Container::Color(color, border_radius) => {
                appearance.background = Some(Background::Color(*color));
                appearance.border_radius = *border_radius;
            }
            _ => {}
        }

        appearance
    }
}

#[derive(Default)]
pub enum Rule {
    #[default]
    Default,

    /// width, length percent
    Width(u16, f32),
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        let mut appearance = rule::Appearance {
            color: Color::from(self.data.theme.spacer),
            width: 1,
            radius: 0.0,
            fill_mode: FillMode::Percent(90.0),
        };

        if let Rule::Width(width, percent) = style {
            appearance.width = *width;
            appearance.radius = *width as f32 / 2.0;
            appearance.fill_mode = FillMode::Percent(*percent);
        }

        appearance
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = (f32, bool);

    fn active(&self, style: &Self::Style) -> scrollable::style::Scrollbar {
        scrollable::style::Scrollbar {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::style::Scroller {
                color: Color::from(if style.1 {
                    self.data.theme.background
                } else {
                    self.data.theme.background_strong2
                }),
                border_radius: style.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::style::Scrollbar {
        self.active(style)
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Color(Color),
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(color) => text::Appearance { color: Some(color) },
        }
    }
}