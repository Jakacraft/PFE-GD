mod player;
mod steam_manager;

use godot::prelude::*;

struct LibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LibExtension {}