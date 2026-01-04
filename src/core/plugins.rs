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

use crate::core::CommandMap;
use crate::core::events::{BroadcastEvent, CommandEvent, DisconnectEvent, OutputEvent};
use crate::core::systems::{flush_broadcasts, flush_output};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CommandEvent>()
            .add_message::<OutputEvent>()
            .add_message::<DisconnectEvent>()
            .add_message::<BroadcastEvent>()
            .insert_resource(CommandMap::new())
            .add_systems(Update, (flush_broadcasts, flush_output).chain());
    }
}
