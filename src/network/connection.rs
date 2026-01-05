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

use crate::core::events::{CommandEvent, DisconnectEvent};

use bevy_ecs::prelude::*;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tracing::{error, info};

const WELCOME_MESSAGE: &str = r#"

                 D R A K O R S     .---._
                                  /==----\_
                 |\              |8=---()-\|       |\
         /|      ::\             |8=-/|\(_)&gt;_    \\
         |:\     `::\_            \=/:\= (_)\|       |\
         `::|     |::::.            \;:\=\(_)&gt;_   |\\
          |b\\_,_ \`::::\             \:\=\( \/       \_(
          `\88b`\|/|'`:::\   .-----   :8:\=|`=&gt;_    [d[
           \;\88b.\=|///::`-/::;;;;:\ |8;|=\( )/       [8[
      __    ||/`888b.\_\/\:/:;/'/-\::\/( \|=(=)&gt;_    [d|
     //):.. `::|/|\"96.\|//;/|'| /-\::+\|-\=(. )/       [8[
    |(/88e::.. `'.|| "min;/\\/8|\.-|::|8|||=|`='_       `[d|
     `-|8888ee::,,,`\/88utes8P//8|-|::|8||\=|( )/        ]8[
      |:`"|####b:::/8pq8e/::'`;q8|/::dP'|\|=|`='_        [d|
 .=-. \::..`""\##Gst:q| e|:/..\:|8|.:/|'/\/|/|(_)/       ]8[
/(,:;:, \::::.\#/88q;`;'\||.:/-//.;/&lt;8\\\^\||./&gt;  `]d[
`8888b::,,_ ::/88q;.`;|d8/`-.]/|./  |8|\|:|;/.d|        [8|
  `"88###n::-/d8P.\e/-|d/ _//;;|/   |8(|::(/).8/       ]d[
    `"\###o2:1dP;`q./=/d/_//|8888\   ;8|^\/`-'8/       [8\
       `"v7|9q8e;./=/d//=/\|eeeb|  /dP= =&lt;ee8/     ]d-
          \|9; qe/-d/ .|/=/888|:\ `--=- =88p'         [8[
          (d5b;,/ d/.|/=-\Oo88|:/                    ,8_\
         _|\q88| d/ /'=q8|888/:/                     ]d|
        _\\\/q8/|8\_""/////eb|/_                     [8_\
        \|\\&lt;==_(;d888b,.////////--._             ]8|
       _/\\\/888p |=""";;;;`eee.////.;-._            ,8p\
      /,\\\/88p\ /==/8/.'88`""""88888eeee`.         ]8|
    .d||8,\/p   /-dp_d8|:8:.d\=\    `""""|=\\      .[8_\
    |8||8||8.-/'d88/8p/|:8:|8b\=\        /|\\|    ]8p|
    |8||8||8b''d.='8p//|:8:'`88b`\       |||||)   [8'\
    `8b:qb.\888:e8P/'/P'8:|:8:`888|      |'\||'  /8p|
     q8q\\qe---_P;'.'P|:8:|:8:|`q/\\     '_///  /8p_\
     _|88b-:==:-d6/P' |8::'|8:| ,|\||    '-=' .d8p/
    |__8Pdb-q888P-'  .:8:| |8:| |/\||\     .-e8p/\|
     .-\888b.__      |:8/' |8:| \ _|;|,-eee8\8\|
     \.-\'88/88/e.e.e|8/|\_--.-.-e8|8|88\8p\|
       .'`-;88]88|8|/':|:\ `q|8|8|8'-\| \|
        `' || || |_/(/|;\)`-\\`--,\|
              `' /v"""' `""""""vVV\

Please enter your name: "#;

pub async fn start_networking(
    addr: String,
    command_tx: mpsc::UnboundedSender<CommandEvent>,
    disconnect_tx: mpsc::UnboundedSender<DisconnectEvent>,
    register_tx: mpsc::UnboundedSender<(mpsc::UnboundedSender<String>, oneshot::Sender<u64>)>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("New connection from {}", peer_addr);

        let command_tx = command_tx.clone();
        let disconnect_tx = disconnect_tx.clone();
        let register_tx = register_tx.clone();
        tokio::spawn(async move {
            if handle_connection(stream, command_tx, disconnect_tx, register_tx)
                .await
                .is_err()
            {
                error!("Error handling connection from {}", peer_addr);
            } else {
                info!("Connection closed: {}", peer_addr);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    command_tx: mpsc::UnboundedSender<CommandEvent>,
    disconnect_tx: mpsc::UnboundedSender<DisconnectEvent>,
    register_tx: mpsc::UnboundedSender<(mpsc::UnboundedSender<String>, oneshot::Sender<u64>)>,
) -> anyhow::Result<()> {
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut line = String::new();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let (resp_tx, resp_rx) = oneshot::channel::<u64>();
    register_tx
        .send((tx.clone(), resp_tx))
        .map_err(|_| anyhow::anyhow!("server shut down"))?;
    let entity_bits = resp_rx
        .await
        .map_err(|_| anyhow::anyhow!("server did not respond"))?;
    let player_entity = Entity::from_bits(entity_bits);
    let write_arc: std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>> =
        std::sync::Arc::new(tokio::sync::Mutex::new(write_half));

    let write_for_forward = write_arc.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let mut w = write_for_forward.lock().await;
            let _ = w.write_all(msg.as_bytes()).await;
            let _ = w.flush().await;
        }
    });

    {
        let mut w = write_arc.lock().await;
        w.write_all(WELCOME_MESSAGE.as_bytes()).await?;
        w.flush().await?;
    }

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            // Connection closed
            if disconnect_tx
                .send(DisconnectEvent {
                    player: player_entity,
                })
                .is_err()
            {
                // Game loop shut down
                break;
            }
            break;
        }

        let input = line.trim_end().to_string();
        if input.eq_ignore_ascii_case("quit") {
            let mut w = write_arc.lock().await;
            let _ = w.write_all(b"Goodbye!\r\n").await;
            let _ = w.flush().await;
            if disconnect_tx
                .send(DisconnectEvent {
                    player: player_entity,
                })
                .is_err()
            {
                // Game loop shut down
            }
            break;
        }

        if command_tx
            .send(CommandEvent {
                player: player_entity,
                input,
            })
            .is_err()
        {
            // Game loop shut down
            break;
        }
    }

    Ok(())
}
