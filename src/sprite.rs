use sdl3::pixels;
use sdl3::render::{WindowCanvas, FRect};


pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

pub trait HasTransform {
    fn transform(&self) -> &Transform;

    #[allow(dead_code)]
    fn set_transform(&mut self, t: Transform);
}


pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub trait HasColor {
    fn color(&self) -> &Color;

    #[allow(dead_code)]
    fn set_color(&mut self, c: Color);
}


pub trait Sprite: HasTransform + HasColor {
    fn render(&mut self, renderer: &mut WindowCanvas) {
        let Transform { x, y, w, h } = self.transform();
        let Color { r, g, b } = self.color();
        let dst = FRect::new(*x, *y, *w, *h);

        renderer.set_draw_color(pixels::Color::RGB(*r, *g, *b));
        renderer.fill_rect(dst).unwrap();
        renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    }
}
