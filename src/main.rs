use pendulum::pendulum::Pendulum;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

mod vector;
mod pendulum;

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p2.update();
        self.p.draw(graphics);
        self.p2.draw(graphics);
        helper.request_redraw();
    }
}

fn main() {
    let window = Window::new_centered("Pendulum", (640, 480)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    window.run_loop(win)
}

