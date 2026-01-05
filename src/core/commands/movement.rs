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

use crate::core::components::{Location, Name, Room, Zone};
use crate::core::events::{BroadcastRoomEvent, OutputEvent};
use crate::core::world::ZoneRegistry;

use bevy_ecs::prelude::*;

#[derive(Copy, Clone)]
pub enum StdExits {
    North,
    South,
    East,
    West,
}

impl StdExits {
    pub fn as_str(&self) -> &'static str {
        match self {
            StdExits::North => "north",
            StdExits::South => "south",
            StdExits::East => "east",
            StdExits::West => "west",
        }
    }

    pub fn as_str_noun(&self) -> &'static str {
        match self {
            StdExits::North => "the north",
            StdExits::South => "the south",
            StdExits::East => "the east",
            StdExits::West => "the west",
        }
    }

    pub fn opposite(&self) -> StdExits {
        match self {
            StdExits::North => StdExits::South,
            StdExits::South => StdExits::North,
            StdExits::East => StdExits::West,
            StdExits::West => StdExits::East,
        }
    }
}

pub fn cmd_east(player: Entity, world: &mut World, full: &str, args: &[&str]) {
    cmd_move(StdExits::East, player, world, full, args);
}

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

pub fn cmd_move(dir: StdExits, player: Entity, world: &mut World, _full: &str, _args: &[&str]) {
    let loc = match world.get::<Location>(player) {
        Some(l) => l.0,
        None => {
            world.write_message(OutputEvent {
                player,
                text: "You are nowhere. (no location set)".to_string(),
            });
            return;
        }
    };

    let room = match world.get_mut::<Room>(loc) {
        Some(r) => r,
        None => {
            world.write_message(OutputEvent {
                player,
                text: "You are in an unknown location.".to_string(),
            });
            return;
        }
    };

    let dir_key = dir.as_str();
    if let Some(&target_ent) = room.exits.get(dir_key) {
        if let Some(mut player_loc) = world.get_mut::<Location>(player) {
            player_loc.0 = target_ent;
        }

        let name = match world.get::<Name>(player) {
            Some(n) => n.0.clone(),
            None => "Someone".to_string(),
        };

        if world.get::<Room>(target_ent).is_some() {
            let text = format!("You go {}.", dir_key);
            world.write_message(OutputEvent { player, text });
            world.write_message(BroadcastRoomEvent {
                from: player,
                room: loc,
                text: format!("{} leaves {}.", name, dir_key),
            });
            world.write_message(BroadcastRoomEvent {
                from: player,
                room: target_ent,
                text: format!("{} arrives from {}.", name, dir.opposite().as_str_noun()),
            });
            return;
        } else {
            world.write_message(OutputEvent {
                player,
                text: "You arrive at an unknown location.".to_string(),
            });
            return;
        }
    }

    world.write_message(OutputEvent {
        player,
        text: format!("You can't go {} from here.", dir_key),
    });
}

pub fn cmd_north(player: Entity, world: &mut World, full: &str, args: &[&str]) {
    cmd_move(StdExits::North, player, world, full, args);
}

pub fn cmd_south(player: Entity, world: &mut World, full: &str, args: &[&str]) {
    cmd_move(StdExits::South, player, world, full, args);
}

pub fn cmd_west(player: Entity, world: &mut World, full: &str, args: &[&str]) {
    cmd_move(StdExits::West, player, world, full, args);
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
