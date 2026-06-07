use godot::prelude::*;
use godot::classes::{CharacterBody3D, Input};
use godot::classes::ICharacterBody3D;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    speed: f64,
    gravity: f32,
    jump_height: f64,
    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        //tweak these variables
        Self {
            speed: 25.0,
            gravity: 20.0,
            jump_height: 30.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();

        godot_print!("velocity at start of frame: {:?}", velocity);

        if !self.base().is_on_floor() {
            velocity.y -= self.gravity * delta as f32;
        }

        velocity.x = 0.0;
        velocity.z = 0.0;

        let input = Input::singleton();

        if input.is_action_pressed("Forward") {
            velocity.z -= self.speed as f32;
        }
        if input.is_action_pressed("Backward") {
            velocity.z += self.speed as f32;
        }
        if input.is_action_pressed("Left") {
            velocity.x -= self.speed as f32;
        }
        if input.is_action_pressed("Right") {
            velocity.x += self.speed as f32;
        }
        // JUMP if on the floor
        if input.is_action_just_pressed("Jump") && self.base().is_on_floor() {
            velocity.y = self.jump_height as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}