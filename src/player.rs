use godot::prelude::*;
use godot::classes::{CharacterBody3D, Input, SpringArm3D, InputEvent, InputEventMouseMotion};
use godot::classes::ICharacterBody3D;
use godot::classes::input::MouseMode;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    speed: f64,
    gravity: f32,
    jump_height: f64,
    #[export]
    mouse_sensitivity: f32,
    #[export]
    min_pitch: f32,
    #[export]
    max_pitch: f32,
    yaw: f32,
    pitch: f32,
    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        //tweak these variables
        Self {
            speed: 25.0,
            gravity: 30.0,
            jump_height: 20.0,
            mouse_sensitivity: 0.005,
            min_pitch: -1.2,
            max_pitch: 0.5,
            yaw: 0.0,
            pitch: 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        let mut mouseinput = Input::singleton();
        mouseinput.set_mouse_mode(MouseMode::CAPTURED);
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        let mut direction = Vector3::ZERO;

        if !self.base().is_on_floor() {
            velocity.y -= self.gravity * delta as f32;
        }

        velocity.x = 0.0;
        velocity.z = 0.0;

        let input = Input::singleton();

        if input.is_action_pressed("Forward") {
            direction.z -= 1.0;
        }
        if input.is_action_pressed("Backward") {
            direction.z += 1.0;
        }
        if input.is_action_pressed("Left") {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("Right") {
            direction.x += 1.0;
        }
        // JUMP if on the floor
        if input.is_action_just_pressed("Jump") && self.base().is_on_floor() {
            velocity.y = self.jump_height as f32;
        }

        let basis = self.base().get_transform().basis;
        let world_direction = basis * direction;

        velocity.x = world_direction.x * self.speed as f32;
        velocity.z = world_direction.z * self.speed as f32;

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        let mut spring_arm = self.base().get_node_as::<SpringArm3D>("CameraPivot/SpringArm3D");
        spring_arm.set_rotation(Vector3::new(self.pitch, 0.0, 0.0));
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() else {return;};
        let relative = mouse_motion.get_relative();

        // Update yaw (horizontal) and pitch (vertical)
        self.yaw -= relative.x * self.mouse_sensitivity;
        self.pitch -= relative.y * self.mouse_sensitivity;

        // Clamp pitch so the camera doesn't flip over the top/bottom
        self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);

        let yaw = self.yaw;
        let _pitch = self.pitch;
        self.base_mut().set_rotation(Vector3::new(0.0, yaw, 0.0));
    }
}