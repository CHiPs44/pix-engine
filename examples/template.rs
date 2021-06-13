use pix_engine::prelude::*;

const TITLE: &str = "Example App";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl AppState for App {
    fn on_start(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    fn on_update(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .build();
    let mut app = App::new();
    engine.run(&mut app)
}
