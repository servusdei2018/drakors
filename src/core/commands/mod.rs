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

pub mod movement;
pub mod social;

use std::collections::HashMap;

use crate::core::events::OutputEvent;

use bevy_ecs::prelude::*;

pub type CommandHandler =
    fn(Entity, &mut World, /* full input */ &str, /* args */ &[&str]);

struct CommandMetadata {
    name: &'static str,
    handler: Option<CommandHandler>,
    description: &'static str,
    aliases: &'static [&'static str],
}

const COMMAND_LIST: &[CommandMetadata] = &[
    CommandMetadata {
        name: "east",
        handler: Some(movement::cmd_east),
        description: "Move east",
        aliases: &["e"],
    },
    CommandMetadata {
        name: "help",
        handler: Some(cmd_help),
        description: "Show this help message",
        aliases: &[],
    },
    CommandMetadata {
        name: "look",
        handler: Some(movement::cmd_look),
        description: "Look around the current room",
        aliases: &["l"],
    },
    CommandMetadata {
        name: "north",
        handler: Some(movement::cmd_north),
        description: "Move north",
        aliases: &["n"],
    },
    CommandMetadata {
        name: "quit",
        handler: None,
        description: "Disconnect from the game",
        aliases: &[],
    },
    CommandMetadata {
        name: "say",
        handler: Some(social::cmd_say),
        description: "Speak aloud to others in the room",
        aliases: &[],
    },
    CommandMetadata {
        name: "shout",
        handler: Some(social::cmd_shout),
        description: "Shout to everyone in your zone",
        aliases: &[],
    },
    CommandMetadata {
        name: "south",
        handler: Some(movement::cmd_south),
        description: "Move south",
        aliases: &["s"],
    },
    CommandMetadata {
        name: "west",
        handler: Some(movement::cmd_west),
        description: "Move west",
        aliases: &["w"],
    },
    CommandMetadata {
        name: "where",
        handler: Some(movement::cmd_where),
        description: "Show which zone you are in",
        aliases: &[],
    },
];

#[derive(Resource)]
pub struct CommandMap {
    pub handlers: HashMap<String, CommandHandler>,
    pub help_text: String,
}

impl CommandMap {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        let mut help_lines = Vec::new();

        for cmd in COMMAND_LIST {
            for &alias in cmd.aliases {
                if let Some(h) = cmd.handler {
                    handlers.insert(alias.to_string(), h);
                }
            }

            let aliases_str = if !cmd.aliases.is_empty() {
                format!(" ({})", cmd.aliases.join(", "))
            } else {
                String::new()
            };
            help_lines.push(format!(
                "  {}{} - {}",
                cmd.name, aliases_str, cmd.description
            ));

            if let Some(h) = cmd.handler {
                handlers.insert(cmd.name.to_string(), h);
            }
        }

        help_lines.sort();
        let help_text = format!("Available commands:\n{}", help_lines.join("\n"));

        Self {
            handlers,
            help_text,
        }
    }
}

fn cmd_help(player: Entity, world: &mut World, _full: &str, _args: &[&str]) {
    if let Some(command_map) = world.get_resource::<CommandMap>() {
        world.write_message(OutputEvent {
            player,
            text: command_map.help_text.clone(),
        });
    }
}
