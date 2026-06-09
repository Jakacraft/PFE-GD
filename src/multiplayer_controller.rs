use godot::prelude::*;
use crate::steam_manager::INCOMING_PACKETS;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MultiplayerController {
    base: Base<Node>,
}

#[godot_api]
impl INode for MultiplayerController {
    fn init(base: Base<Node>) -> Self { Self { base } }
    fn process(&mut self, _delta: f64) {
        let mut packets = Vec::new();

        if let Ok(mut lock) = INCOMING_PACKETS.lock() {
            if !lock.is_empty() {
                std::mem::swap(&mut *lock, &mut packets);
            }
        }

        for (sender_id, data) in packets {
            if let Ok(message_str) = std::str::from_utf8(&data) {
                godot_print!("Received packet from [{:?}]: {}", sender_id, message_str);

                // TODO: Update peer positions, spawn characters, or apply damage here
            }
        }
    }
}
