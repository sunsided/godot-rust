use godot::classes::{ISprite2D, Sprite2D};
use godot::global::Key;
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        godot_print!("[Rust]      Init level {:?}", level);
    }

    fn on_level_deinit(level: InitLevel) {
        godot_print!("[Rust]      Deinit level {:?}", level);
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        // TODO: is_action_pressed
        if input.is_key_pressed(Key::W) {
            self.increase_speed(10.0);
        } else if input.is_key_pressed(Key::S) {
            self.increase_speed(-10.0);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        godot_print!("[Rust]      Changed speed to {:?}", self.speed);
        self.base_mut().emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}
