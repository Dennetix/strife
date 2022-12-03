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
    pub spacer: [f32; 4],
    pub primary: [f32; 3],
    pub secondary: [f32; 3],
    pub background: [f32; 3],
    pub background_strong1: [f32; 3],
    pub background_strong2: [f32; 3],
    pub background_weak: [f32; 3],
    pub button: Button,
}

impl DefaultThemes for Theme {
    fn dark() -> Self {
        Self {
            text: [1.0, 1.0, 1.0],
            spacer: [1.0, 1.0, 1.0, 0.05],
            primary: [0.2, 0.5, 1.0],
            secondary: [0.55, 0.55, 0.55],
            background: [0.18, 0.18, 0.18],
            background_strong1: [0.15, 0.15, 0.15],
            background_strong2: [0.12, 0.12, 0.12],
            background_weak: [0.35, 0.35, 0.35],
            button: Button::dark(),
        }
    }

    fn light() -> Self {
        Self {
            text: [0.1, 0.1, 0.1],
            spacer: [0.0, 0.0, 0.0, 0.1],
            primary: [0.2, 0.5, 1.0],
            secondary: [0.65, 0.65, 0.65],
            background: [1.0, 1.0, 1.0],
            background_strong1: [0.87, 0.87, 0.87],
            background_strong2: [0.78, 0.78, 0.78],
            background_weak: [0.85, 0.85, 0.85],
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
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub text: [f32; 3],
    pub primary_hovered: [f32; 3],
    pub primary_pressed: [f32; 3],
    pub secondary_hovered: [f32; 3],
    pub secondary_pressed: [f32; 3],
    pub transparent_hover: [f32; 4],
    pub transparent_pressed: [f32; 4],
    pub disabled: [f32; 3],
    pub border_radius: f32,
}

impl DefaultThemes for Button {
    fn dark() -> Self {
        Self {
            text: [1.0, 1.0, 1.0],
            primary_hovered: [0.1, 0.4, 0.9],
            primary_pressed: [0.05, 0.35, 0.85],
            secondary_hovered: [0.15, 0.15, 0.15],
            secondary_pressed: [0.17, 0.17, 0.17],
            transparent_hover: [1.0, 1.0, 1.0, 0.07],
            transparent_pressed: [1.0, 1.0, 1.0, 0.1],
            disabled: [0.7, 0.8, 0.8],
            border_radius: 2.0,
        }
    }

    fn light() -> Self {
        Self {
            text: [0.1, 0.1, 0.1],
            primary_hovered: [0.1, 0.4, 0.9],
            primary_pressed: [0.05, 0.35, 0.85],
            secondary_hovered: [0.75, 0.75, 0.75],
            secondary_pressed: [0.7, 0.7, 0.7],
            transparent_hover: [0.0, 0.0, 0.0, 0.2],
            transparent_pressed: [0.0, 0.0, 0.0, 0.3],
            disabled: [0.7, 0.8, 0.8],
            border_radius: 2.0,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::dark()
    }
}
