#![windows_subsystem = "windows"]

mod game;
mod player;

use game::{Grid, COLUMNS};
use log::{error, info, warn};
use player::{MsgCode, Player, Turn};
use rand::random;

use std::thread;
use websocket::sync::Server;

fn main() {
    // Initialize the logging API
    log4rs::init_file("log/config.yml", Default::default()).unwrap();

    let server: Server<_> = Server::bind("0.0.0.0:4730").unwrap();

    // Holder for the player that is waiting for another one to connect
    let mut pending: Option<Player> = None;

    for request in server.filter_map(Result::ok) {
        // Reject the request if the clients doesn't support the connect4 sub-protocol
        if !request.protocols().contains(&"connect4".into()) {
            request.reject().unwrap();
        }
        // Otherwise accept the client request
        else {
            let client = request.use_protocol("connect4").accept().unwrap();
            info!("CLIENT CONNECTED <{}>", client.peer_addr().unwrap());

            // If there is already a player waiting, start the game
            if let Some(player) = pending {
                pending = None;

                // Spawn a new thread for each game
                thread::spawn(|| {
                    let mut player: [Player; 2] = [player, client.into()];
                    info!("GAME STARTED : PLAYERS <{}> <{}>", player[0], player[1]);

                    // Decide randomly who starts and comunicate the decision
                    let mut turn = if random() {
                        player[0].send_response(MsgCode::First);
                        player[1].send_response(MsgCode::Second);
                        Turn::A
                    } else {
                        player[1].send_response(MsgCode::First);
                        player[0].send_response(MsgCode::Second);
                        Turn::B
                    };
                    // Create a new connect four grid
                    let mut grid = Grid::new();

                    let mut end = false;
                    while !end {
                        let current = turn as usize;
                        let other = turn.flipped() as usize;
                        // Recive the player input (the column number) and check for errors
                        if let Ok(option) = player[current].get_column() {
                            if let Some(column) = option {
                                // Check if the specified column exists
                                if column < COLUMNS {
                                    // Insert the disc in the specified column
                                    if grid.insert_disc(column as usize, turn) {
                                        // If the current player won, comunicate the end of the game
                                        if grid.is_win(column as usize, turn) {
                                            player[current].send_response(MsgCode::Win);
                                            player[other].send_response(MsgCode::Lose);
                                            player[other].send_column(column);
                                            turn.flip();
                                            end = true;
                                        }
                                        // If the game grid is full, comunicate the end of the game
                                        else if grid.is_full() {
                                            // No one won
                                            player[current].send_response(MsgCode::Draw);
                                            player[other].send_response(MsgCode::Draw);
                                            end = true;
                                        } else {
                                            // End the turn of this player and start the other player one
                                            player[current].send_response(MsgCode::Wait);
                                            player[other].send_response(MsgCode::Go);
                                            player[other].send_column(column);
                                            turn.flip();
                                        }
                                    // If the column specified is full, inform the player
                                    } else {
                                        player[current].send_response(MsgCode::ColumnFull);
                                    }
                                // If the column doesn't exist, inform the player
                                } else {
                                    player[current].send_response(MsgCode::OutOfBounds);
                                }
                            // If the data recived is not a number, inform the player
                            } else {
                                player[current].send_response(MsgCode::Unexpected);
                            }
                        // If there was an error tell the other player the current one left
                        } else {
                            player[other].send_response(MsgCode::OtherLeft);
                            end = true;

                            error!("PLAYER {} DISCONNECTED", player[current]);
                        }
                    }
                    info!("GAME ENDED : PLAYERS <{}> <{}>", player[0], player[1]);
                });
            }
            // Else if there is noone, put the connected client on hold
            else {
                pending = Some(client.into());
            }
        }
    }
}
// Prints a message in case of an error
fn ignore_error<T, E: std::fmt::Debug>(result: Result<T, E>) {
    if let Err(e) = result {
        warn!("INGORE '{:?}'", e);
    }
}
