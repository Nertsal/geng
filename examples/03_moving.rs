use geng::prelude::*;

struct State {
    geng: Geng,
    position: vec2<f32>, // Current position
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            position: vec2::ZERO,
        }
    }
}

impl geng::State for State {
    // Specify how to update game state over time
    fn update(
        &mut self,
        delta_time: f64, // Time in seconds since last update
    ) {
        let delta_time = delta_time as f32;

        // Move depending on the keys currently being pressed
        if self.geng.window().is_key_pressed(geng::Key::ArrowLeft) {
            self.position.x -= delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::ArrowRight) {
            self.position.x += delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::ArrowUp) {
            self.position.y += delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::ArrowDown) {
            self.position.y -= delta_time;
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: Angle::ZERO,
                fov: Camera2dFov::Vertical(15.0),
            },
            "Use arrow keys to move around\nPress Space to reset",
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(self.position),
            Rgba::WHITE,
        );
    }
    // We can handle events like KeyDown by implementing this method
    fn handle_event(&mut self, event: geng::Event) {
        if matches!(
            event,
            geng::Event::KeyPress {
                key: geng::Key::Space
            }
        ) {
            self.position = vec2::ZERO;
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("Moving", |geng| async move {
        geng.run_state(State::new(&geng)).await;
    });
}
