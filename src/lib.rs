mod player;

use godot::prelude::*;

struct LibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LibExtension {}