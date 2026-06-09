use godot::prelude::*;
use once_cell::sync::OnceCell;
use steamworks::{Client, SteamId, networking_types, LobbyEnter, LobbyId};
use std::sync::Mutex;
use steamworks::networking_types::SendFlags;

static STEAM_CLIENT: OnceCell<Client> = OnceCell::new();
pub static INCOMING_PACKETS: Mutex<Vec<(SteamId, Vec<u8>)>> = Mutex::new(Vec::new());
pub static CURRENT_LOBBY: Mutex<Option<LobbyId>> = Mutex::new(None);
#[derive(GodotClass)]
#[class(base=Node)]
pub struct SteamManager {
    _callback_handles: Vec<steamworks::CallbackHandle>,
    base: Base<Node>
}
#[godot_api]
impl INode for SteamManager {
    fn init(base: Base<Node>) -> Self {
        Self {_callback_handles: Vec::new(), base}
    }
    fn ready(&mut self) {
        match Client::init_app(4503830) {
            Ok(client) =>
                {godot_print!("Steam Reporting For Duty Sir!");
                let join_handle = client.register_callback(|event: LobbyEnter| {
                   let lobby_id = event.lobby;
                    godot_print!("Lobby Id: {:?}", lobby_id);
                    if let Ok(mut lock) = CURRENT_LOBBY.lock() {
                        *lock = Some(lobby_id);
                    }
                    // TODO: Load the level here w/ a godot signal
                });
                self._callback_handles.push(join_handle);
                let _ = STEAM_CLIENT.set(client);
                if let Some(steam) = STEAM_CLIENT.get() {
                    let name = steam.friends().name();
                    godot_print!("Steam Reporting New Member: {}", name);
                }
            }
            Err(e) => {
                godot_print!("Steam has the boo-boo: {:?}", e);
            }
        }
    }
    fn process(&mut self, _delta: f64) {
        if let Some(steam) = STEAM_CLIENT.get() {
            steam.run_callbacks();
            let net_messages = steam.networking_messages();
            let messages: Vec<networking_types::NetworkingMessage> =
                net_messages.receive_messages_on_channel(0, 32);
            if !messages.is_empty() {
                if let Ok(mut lock) = INCOMING_PACKETS.lock() {
                    for msg in messages {
                        let identity = msg.identity_peer();
                        if let Some(sender) = identity.steam_id() {
                            let data = msg.data().to_vec();
                            lock.push((sender, data));
                            }
                        }
                    }
                }
            }
        }
    }
pub fn get_steam() -> Option<&'static Client> {
    STEAM_CLIENT.get()
}
pub fn send_p2p_message(target_peer: SteamId, payload: &[u8]) {
    if let Some(steam) = get_steam() {
        let net_mesages = steam.networking_messages();
        let send_result = net_mesages.send_message_to_user(
            target_peer.into(),
            SendFlags::RELIABLE,
            payload,
            0,
        );
        if let Err(e) = send_result {
            godot_print!("The Payload is Gone!: {:?}", e);
        }
    }
}
/// Step A: Call this to create a matchmaking room on Steam's backend servers
pub fn create_steam_lobby() {
    if let Some(steam) = get_steam() {
        let matchmaking = steam.matchmaking();
        matchmaking.create_lobby(steamworks::LobbyType::Public, 4, move |result| {
            match result {
                Ok(lobby_id) => {
                    godot_print!("Lobby created successfully backend! ID: {:?}", lobby_id);

                    if let Ok(mut lock) = CURRENT_LOBBY.lock() {
                        *lock = Some(lobby_id);
                    }
                }
                Err(e) => godot_print!("Failed to create Steam Lobby: {:?}", e),
            }
        });
    }
}
pub fn open_invite_overlay() {
    if let Some(steam) = get_steam() {
        if let Ok(lock) = CURRENT_LOBBY.lock() {
            if let Some(lobby_id) = *lock {
                steam.friends().activate_invite_dialog(lobby_id);
                godot_print!("Opening Steam invitation dialogue overlay.");
            } else {
                godot_print!("Cannot invite friends: You must create a lobby first!");
            }
        }
    }
}