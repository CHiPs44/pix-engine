use pix_engine::prelude::*;

struct HelloWorld;

impl AppState for HelloWorld {
    // Set up any state or resources before starting main event loop.
    fn on_start(&mut self, s: &mut PixState) -> Result<()> {
        s.background(220);
        Ok(())
    }

    // Main render loop. Called as often as possible, or based on `target_frame_rate`.
    fn on_update(&mut self, s: &mut PixState) -> Result<()> {
        s.clear()?;
        s.text("Hello world!")?;
        Ok(())
    }

    // Teardown any state or resources before exiting.
    fn on_stop(&mut self, _s: &mut PixState) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = HelloWorld;
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("Hello World")
        .build()?;
    engine.run(&mut app)
}
