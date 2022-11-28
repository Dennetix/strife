use serde::Deserialize;

pub trait DefaultThemes: Default {
    fn dark() -> Self;
    fn light() -> Self;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeData {
    pub id: String,
    pub name: String,
    pub theme: Theme,
}

impl DefaultThemes for ThemeData {
    fn dark() -> Self {
        Self {
            id: String::from("strife.dark"),
            name: String::from("Default Dark"),
            theme: Theme::dark(),
        }
    }

    fn light() -> Self {
        Self {
            id: String::from("strife.light"),
            name: String::from("Default Light"),
            theme: Theme::light(),
        }
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        Self::dark()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub text: [f32; 3],
    pub background: [f32; 3],
    pub background_contrast1: [f32; 3],
    pub background_contrast2: [f32; 3],
    pub button: Button,
}

impl DefaultThemes for Theme {
    fn dark() -> Self {
        Self {
            text: [1.0, 1.0, 1.0],
            background: [0.1, 0.1, 0.1],
            background_contrast1: [0.15, 0.15, 0.15],
            background_contrast2: [0.2, 0.2, 0.2],
            button: Button::dark(),
        }
    }

    fn light() -> Self {
        Self {
            text: [0.1, 0.1, 0.1],
            background: [1.0, 1.0, 1.0],
            background_contrast1: [0.9, 0.9, 0.9],
            background_contrast2: [0.8, 0.8, 0.8],
            button: Button::light(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Button {
    pub active: [f32; 3],
    pub disabled: [f32; 3],
    pub hovered: [f32; 3],
    pub pressed: [f32; 3],
}

impl DefaultThemes for Button {
    fn dark() -> Self {
        Self {
            active: [0.2, 0.5, 1.0],
            disabled: [0.7, 0.8, 0.8],
            hovered: [0.1, 0.4, 0.9],
            pressed: [0.05, 0.35, 0.85],
        }
    }

    fn light() -> Self {
        Self {
            active: [0.2, 0.5, 1.0],
            disabled: [0.7, 0.8, 0.8],
            hovered: [0.1, 0.4, 0.9],
            pressed: [0.05, 0.35, 0.85],
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::dark()
    }
}
