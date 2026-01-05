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

use std::collections::HashMap;
use std::fs;

use crate::core::components::Room;

use anyhow::Context;
use bevy_ecs::prelude::World;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use toml;

#[derive(Resource, Default)]
pub struct RoomRegistry {
    pub id_to_entity: HashMap<String, Entity>,
}

impl RoomRegistry {
    pub fn insert(&mut self, id: String, ent: Entity) {
        self.id_to_entity.insert(id, ent);
    }

    pub fn get(&self, id: &str) -> Option<Entity> {
        self.id_to_entity.get(id).copied()
    }
}

#[derive(Resource, Default)]
pub struct ZoneRegistry {
    pub id_to_name: HashMap<String, String>,
}

pub fn create_room(world: &mut World, name: &str, description: &str) -> Entity {
    let room = Room {
        name: name.to_string(),
        description: description.to_string(),
        exits: HashMap::new(),
    };

    let ent = world.spawn((room,)).id();
    ent
}

#[derive(Deserialize, Debug)]
pub struct RoomDef {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub exits: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub rooms: Vec<RoomDef>,
}

pub fn load_zones_from_dir(world: &mut World, dir: &str) -> anyhow::Result<()> {
    let paths = fs::read_dir(dir).with_context(|| format!("reading zone dir {}", dir))?;

    for entry in paths {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }

        let contents = fs::read_to_string(&path).with_context(|| format!("reading {:?}", path))?;
        let zone: ZoneDef =
            toml::from_str(&contents).with_context(|| format!("parsing {:?}", path))?;

        let mut local_map: HashMap<String, Entity> = HashMap::new();
        for r in &zone.rooms {
            let ent = create_room(world, &r.name, &r.description);
            let _ = world
                .entity_mut(ent)
                .insert(crate::core::components::Zone(zone.id.clone()));
            local_map.insert(r.id.clone(), ent);
        }

        for r in &zone.rooms {
            if let Some(&ent) = local_map.get(&r.id) {
                if let Some(mut room) = world.get_mut::<crate::core::components::Room>(ent) {
                    for (dir_str, target_id) in &r.exits {
                        if let Some(&target_ent) = local_map.get(target_id) {
                            room.exits.insert(dir_str.clone(), target_ent);
                        }
                    }
                }
            }
        }

        let mut reg = world.resource_mut::<RoomRegistry>();
        for (id, ent) in local_map {
            let reg_id = format!("{}:{}", zone.id, id);
            reg.insert(reg_id, ent);
        }

        let mut zin = world.resource_mut::<ZoneRegistry>();
        zin.id_to_name.insert(zone.id.clone(), zone.name.clone());
    }

    Ok(())
}
