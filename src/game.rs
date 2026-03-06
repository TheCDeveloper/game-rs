use crate::sprite::Sprite;
use crate::player::Player;

use sdl3::{Sdl, EventPump, VideoSubsystem};
use sdl3::render::WindowCanvas;


pub struct Game {
    _sdl: Sdl,
    events: EventPump,
    _video: VideoSubsystem,
    renderer: WindowCanvas,

    player: Player
}


impl Game {
    pub fn new() -> Result<Game, sdl3::Error> {
        let sdl = sdl3::init()?;
        let events = sdl.event_pump()?;
        let video = sdl.video()?;
        let renderer = video.window_and_renderer("Flappy-square", 800, 500)?;

        Ok(Game {
            _sdl: sdl,
            events: events,
            _video: video,
            renderer: renderer,

            player: Player::new()
        })
    }


    pub fn run(&mut self) {
        'mainloop: loop {
            self.renderer.clear();
            self.player.render(&mut self.renderer);
            self.renderer.present();

            for e in self.events.poll_iter() {
                use sdl3::event::Event::*;

                match e {
                    Quit{..} => {
                        break 'mainloop;
                    },

                    _ => {}
                }
            }
        }
    }
}
