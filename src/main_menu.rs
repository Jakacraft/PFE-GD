use godot::prelude::*;
use crate::steam_manager::{create_steam_lobby, open_invite_overlay};

#[derive(GodotClass)]
#[class(base=Node)]
struct MainMenuController {
    base: Base<Node>,
}

#[godot_api]
impl INode for MainMenuController {
    fn init(base: Base<Node>) -> Self { Self { base } }
}

#[godot_api]
impl MainMenuController {
    #[func]
    fn _on_host_button_pressed(&self) {
        create_steam_lobby();
    }

    #[func]
    fn _on_invite_button_pressed(&self) {
        open_invite_overlay();
    }
}