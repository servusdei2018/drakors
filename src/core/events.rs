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

use bevy_ecs::prelude::*;

#[derive(Message)]
pub struct BroadcastEvent {
    pub from: Entity,
    pub text: String,
}

#[derive(Message)]
pub struct BroadcastRoomEvent {
    pub from: Entity,
    pub room: Entity,
    pub text: String,
}

#[derive(Message)]
pub struct BroadcastZoneEvent {
    pub from: Entity,
    pub zone: String,
    pub text: String,
}

#[derive(Message)]
pub struct CommandEvent {
    pub player: Entity,
    pub input: String,
}

#[derive(Message)]
pub struct DisconnectEvent {
    pub player: Entity,
}

#[derive(Message)]
pub struct OutputEvent {
    pub player: Entity,
    pub text: String,
}
