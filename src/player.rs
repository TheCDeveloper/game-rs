use crate::sprite::*;


pub struct Player {
    transform: Transform,
    color: Color
}


impl HasTransform for Player {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }
}

impl HasColor for Player {
    fn color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, c: Color) {
        self.color = c;
    }
}

impl Sprite for Player {}

impl Player {
    pub fn new() -> Player {
        Player {
            transform: Transform { x: 0.0, y: 0.0, w: 100.0, h: 100.0 },
            color: Color { r: 255, g: 0, b: 0 }
        }
    }
}
