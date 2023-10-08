pub mod pendulum {
    use speedy2d::{Graphics2D, color::Color};

    use crate::vector::vector::Vector;

    pub struct Pendulum {
        pub angle: f32,
        pub angular_velocity: f32,
        pub angular_acceleration: f32,
        pub length: f32,
        pub mass: f32,
        pub gravity: f32,

        pub origin: Vector,
        pub position: Vector,
    }

    impl Pendulum {
        pub fn new(x: f32, y: f32, length: f32) -> Pendulum {
            Pendulum {
                angle: 1.0,
                angular_acceleration: 0.0,
                angular_velocity: 0.0,
                gravity: 1.5,
                length,
                mass: 1.0,
                origin: Vector::new(x, y),
                position: Vector::new(0.0, 0.0),
            }
        }

        pub fn update(&mut self) {
            self.angular_acceleration = (-1.0 * self.gravity * self.angle.sin()) / self.length;
            self.angular_velocity += self.angular_acceleration;
            self.angle += self.angular_velocity;
            self.position.set(
                self.length * self.angle.sin(),
                self.length * self.angle.cos(),
            );
            self.position.add(&self.origin);
        }

        pub fn draw(&self, graphics: &mut Graphics2D) {
            graphics.draw_line(
                (self.origin.x, self.origin.y),
                (self.position.x, self.position.y),
                3.0,
                Color::RED
            );

            graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
        }
    }
}
