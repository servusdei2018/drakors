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

use crate::core::{
    components::{Name, OutputTx, Player},
    events::{BroadcastEvent, OutputEvent},
};

use bevy_ecs::prelude::*;

/// Broadcasts messages to all players except the sender
pub fn flush_broadcasts(
    players: Query<(Entity, &OutputTx, Option<&Name>), With<Player>>,
    mut events: MessageReader<BroadcastEvent>,
) {
    for event in events.read() {
        for (ent, tx, maybe_name) in players.iter() {
            if ent == event.from {
                continue;
            }
            if maybe_name.is_some() {
                let _ = tx.0.send(format!("{}\r\n> ", event.text));
            }
        }
    }
}

/// Sends output messages to individual players
pub fn flush_output(
    mut players: Query<&OutputTx, With<Player>>,
    mut events: MessageReader<OutputEvent>,
) {
    for event in events.read() {
        if let Ok(tx) = players.get_mut(event.player) {
            let _ = tx.0.send(format!("{}\r\n> ", event.text));
        }
    }
}
