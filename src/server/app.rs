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
use crate::core::CorePlugin;
use crate::core::components::{Name, OutputTx, Player, PlayerState};
use crate::core::events::{BroadcastEvent, CommandEvent, DisconnectEvent, OutputEvent};
use crate::network::connection::start_networking;

use bevy_app::App;
use bevy_ecs::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tracing::info;

#[derive(Resource)]
pub struct CommandQueue(pub mpsc::UnboundedReceiver<CommandEvent>);

#[derive(Resource)]
pub struct DisconnectQueue(pub mpsc::UnboundedReceiver<DisconnectEvent>);

pub async fn run_server(addr: &str) -> anyhow::Result<()> {
    let (command_tx, command_rx) = mpsc::unbounded_channel::<CommandEvent>();
    let (disconnect_tx, disconnect_rx) = mpsc::unbounded_channel::<DisconnectEvent>();

    let (register_tx, mut register_rx) = mpsc::unbounded_channel::<(
        tokio::sync::mpsc::UnboundedSender<String>,
        oneshot::Sender<u64>,
    )>();

    let mut network_handle = tokio::spawn(start_networking(
        addr.to_string(),
        command_tx,
        disconnect_tx,
        register_tx.clone(),
    ));

    let mut app = App::new();
    app.add_plugins(CorePlugin)
        .insert_resource(CommandQueue(command_rx))
        .insert_resource(DisconnectQueue(disconnect_rx));

    info!("Drakors starting on {}", addr);

    let tick_duration = std::time::Duration::from_millis(50); // 20 ticks per second (50ms per tick)
    let mut tick_timer = tokio::time::interval(tick_duration);

    loop {
        tokio::select! {
            _ = tick_timer.tick() => {
                // Handle commands
                while let Ok(event) = app.world_mut().resource_mut::<CommandQueue>().0.try_recv() {
                    let input = event.input.trim().to_string();

                    if let Some(state) = app.world().get::<PlayerState>(event.player) {
                        if *state == PlayerState::ChoosingName {
                            let existing: Vec<String> = {
                                let world = app.world_mut();
                                world
                                    .query::<&Name>()
                                    .iter(&world)
                                    .map(|n| n.0.clone())
                                    .collect()
                            };

                            if existing.iter().any(|n| n == &input) {
                                app.world_mut().write_message(OutputEvent {
                                    player: event.player,
                                    text: "Name already in use, please pick another".to_string(),
                                });
                            } else {
                                app.world_mut().entity_mut(event.player).insert(Name(input.clone()));
                                app.world_mut().entity_mut(event.player).insert(PlayerState::Active);
                                app.world_mut().write_message(BroadcastEvent {
                                    from: event.player,
                                    text: format!("{} has joined the game.", input.clone()),
                                });
                                app.world_mut().write_message(OutputEvent {
                                    player: event.player,
                                    text: format!("Welcome, {}!", input.clone()),
                                });
                            }
                            continue;
                        }
                    }

                    let mut words: Vec<&str> = input.split_whitespace().collect();
                    let command_name = if words.is_empty() {
                        "look".to_string()
                    } else {
                        words.remove(0).to_lowercase()
                    };
                    let args: Vec<&str> = words;

                    let handler_opt = {
                        let map = app.world().resource::<CommandMap>();
                        map.handlers.get(&command_name).copied()
                    };
                    if let Some(handler) = handler_opt {
                        handler(event.player, app.world_mut(), &input, &args);
                    } else {
                        app.world_mut().write_message(OutputEvent {
                            player: event.player,
                            text: format!("Unknown command: {}", command_name),
                        });
                    }
                }

                // Handle disconnects
                while let Ok(event) = app.world_mut().resource_mut::<DisconnectQueue>().0.try_recv() {
                    let maybe_name = app.world().get::<Name>(event.player).map(|n| n.0.clone());
                    if let Some(name) = maybe_name {
                        app.world_mut().write_message(BroadcastEvent {
                            from: event.player,
                            text: format!("{} has left the game.", name),
                        });
                    }
                    app.world_mut().despawn(event.player);
                }

                // Handle incoming player registration
                while let Ok((tx, resp)) = register_rx.try_recv() {
                    let entity = app
                        .world_mut()
                        .spawn((Player, OutputTx(tx.clone()), PlayerState::ChoosingName))
                        .id();

                    let _ = resp.send(entity.to_bits());
                }

                app.update();
            }

            // Handle network task completion (should not happen normally)
            res = &mut network_handle => {
                res?;
                break;
            }
        }
    }

    Ok(())
}
