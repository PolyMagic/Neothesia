use crate::{
    config::ColorSchemaV1,
    render::QuadInstance,
    utils::{Point, Size},
};
use wgpu_jumpstart::Color;

#[derive(Debug, Clone)]
pub struct KeyState {
    is_sharp: bool,

    pressed_by_file: Option<Color>,
    pressed_by_user: bool,
}

impl KeyState {
    pub fn new(is_sharp: bool) -> Self {
        Self {
            is_sharp,

            pressed_by_file: None,
            pressed_by_user: false,
        }
    }

    pub fn pressed_by_file(&self) -> Option<&Color> {
        self.pressed_by_file.as_ref()
    }

    pub fn set_pressed_by_user(&mut self, is: bool) {
        self.pressed_by_user = is;
    }

    pub fn pressed_by_file_on(&mut self, scheme: &ColorSchemaV1) {
        let (r, g, b) = if self.is_sharp {
            scheme.dark
        } else {
            scheme.base
        };

        self.pressed_by_file = Some(Color::from_rgba8(r, g, b, 1.0));
    }

    pub fn pressed_by_file_off(&mut self) {
        self.pressed_by_file = None;
    }

    pub fn color(&self) -> Color {
        if self.pressed_by_user {
            let v = if self.is_sharp { 0.3 } else { 0.5 };
            Color::new(v, v, v, 1.0)
        } else if let Some(color) = self.pressed_by_file {
            color
        } else if self.is_sharp {
            Color::new(0.0, 0.0, 0.0, 1.0)
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

pub fn border_radius(w: f32, is_sharp: bool) -> f32 {
    let kind_multiplier = if is_sharp { 2.0 } else { 3.5 };

    let radius = w * 0.08;

    radius * kind_multiplier
}

pub fn to_quad(key: &piano_layout::Key, color: Color, origin: Point<f32>) -> QuadInstance {
    let position = [origin.x + key.x(), origin.y];

    let mut size: Size<f32> = key.size().into();

    if let piano_layout::KeyKind::Neutral = key.kind() {
        size.width -= 1.0;
    }

    let r = border_radius(size.width, key.kind().is_sharp());

    QuadInstance {
        position,
        size: size.into(),
        color: color.into_linear_rgba(),
        border_radius: [0.0, 0.0, r, r],
    }
}
