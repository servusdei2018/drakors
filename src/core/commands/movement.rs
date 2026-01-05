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

use crate::core::components::{Location, Room, Zone};
use crate::core::events::OutputEvent;
use crate::core::world::ZoneRegistry;

use bevy_ecs::prelude::*;

pub fn cmd_look(player: Entity, world: &mut World, _full: &str, _args: &[&str]) {
    if let Some(loc) = world.get::<Location>(player) {
        if let Some(room) = world.get::<Room>(loc.0) {
            let text = format!("{}\n\n{}", room.name, room.description);
            world.write_message(OutputEvent { player, text });
            return;
        }
    }

    let text = "You are nowhere. (no location set)";
    world.write_message(OutputEvent {
        player,
        text: text.to_string(),
    });
}

pub fn cmd_where(player: Entity, world: &mut World, _full: &str, _args: &[&str]) {
    if let Some(loc) = world.get::<Location>(player) {
        if let Some(zone_comp) = world.get::<Zone>(loc.0) {
            let zone_id = &zone_comp.0;
            if let Some(zin) = world.get_resource::<ZoneRegistry>() {
                if let Some(zone_name) = zin.id_to_name.get(zone_id) {
                    let text = format!("You are in: {}", zone_name);
                    world.write_message(OutputEvent { player, text });
                    return;
                }
            }
        }
    }

    let text = "You are nowhere. (zone unknown)";
    world.write_message(OutputEvent {
        player,
        text: text.to_string(),
    });
}
