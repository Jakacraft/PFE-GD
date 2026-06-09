mod player;
mod steam_manager;
mod multiplayer_controller;
mod main_menu;

use godot::prelude::*;

struct LibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LibExtension {}