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

use crate::core::events::OutputEvent;

use bevy_ecs::prelude::*;

pub fn cmd_look(player: Entity, world: &mut World, _full: &str, _args: &[&str]) {
    let text = "You look around and see... the vast potential of a growing MUD.";
    world.write_message(OutputEvent {
        player,
        text: text.to_string(),
    });
}
