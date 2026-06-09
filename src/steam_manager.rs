use godot::prelude::*;
use once_cell::sync::OnceCell;
use steamworks::Client;

static STEAM_CLIENT: OnceCell<Client> = OnceCell::new();
#[derive(GodotClass)]
#[class(base=Node)]
pub struct SteamManager {
    base: Base<Node>
}
#[godot_api]
impl INode for SteamManager {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        match Client::init_app(4503830) {
            Ok(client) =>
                {godot_print!("Steam Reporting For Duty Sir!");
                let _ = STEAM_CLIENT.set(client);
                if let Some(steam) = STEAM_CLIENT.get() {
                    let name = steam.friends().name();
                    godot_print!("Steam Reporting New Member: {}", name);
                }
            }
            Err(e) => {
                godot_print!("Steam has the booboo: {:?}", e);
            }
        }
    }
    fn process(&mut self, _delta: f64) {
        if let Some(steam) = STEAM_CLIENT.get() {
            steam.run_callbacks();
        }
    }
}
pub fn get_steam() -> Option<&'static Client> {
    STEAM_CLIENT.get()
}