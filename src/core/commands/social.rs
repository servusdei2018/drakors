// Drakors
// Copyright (C) 2025-present  Nathanael Bracy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::core::components::{Location, Name, Zone};
use crate::core::events::{BroadcastRoomEvent, BroadcastZoneEvent, OutputEvent};

use bevy_ecs::prelude::*;

pub fn cmd_say(player: Entity, world: &mut World, _full: &str, args: &[&str]) {
    if args.is_empty() {
        world.write_message(OutputEvent {
            player,
            text: "Say what?".to_string(),
        });
        return;
    }

    let message = args.join(" ");
    let name = match world.get::<Name>(player) {
        Some(n) => n.0.clone(),
        None => "Someone".to_string(),
    };

    if let Some(loc) = world.get::<Location>(player) {
        world.write_message(BroadcastRoomEvent {
            from: player,
            room: loc.0,
            text: format!("{} says: {}", name, message),
        });
    } else {
        world.write_message(OutputEvent {
            player,
            text: "Your words echo into the void... there is no one around to hear them."
                .to_string(),
        });
        return;
    }
    world.write_message(OutputEvent {
        player,
        text: format!("You say: {}", message),
    });
}

pub fn cmd_shout(player: Entity, world: &mut World, _full: &str, args: &[&str]) {
    if args.is_empty() {
        world.write_message(OutputEvent {
            player,
            text: "Shout what?".to_string(),
        });
        return;
    }

    let message = args.join(" ");
    let player_zone = if let Some(loc) = world.get::<Location>(player) {
        if let Some(zone) = world.get::<Zone>(loc.0) {
            zone.0.clone()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    let name = match world.get::<Name>(player) {
        Some(n) => n.0.clone(),
        None => "Someone".to_string(),
    };

    world.write_message(BroadcastZoneEvent {
        from: player,
        zone: player_zone,
        text: format!("{} shouts: {}", name, message),
    });
    world.write_message(OutputEvent {
        player,
        text: format!("You shout: {}", message),
    });
}
