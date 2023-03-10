use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    messages::*,
    room::{self, GameState, GameStatus, Player, Room},
};

pub struct Server {
    sessions: HashMap<Uuid, Recipient<ClientServerMessage>>,
    rooms: HashMap<Uuid, Room>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Server {
    pub fn new() -> Self {
        let test_room = Room {
            room_id: Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            status: room::GameStatus::WAITING,
            players: BTreeMap::new(),
            owner: Uuid::new_v4(),
            state: GameState::default(),
        };

        let mut rooms = HashMap::new();

        rooms.insert(
            Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            test_room,
        );
        Server {
            sessions: HashMap::new(),
            rooms,
        }
    }

    pub fn send_message(
        &self,
        room: &Uuid,
        msg_type: MessageTypes,
        content: String,
        skip_id: Option<&Uuid>,
    ) {
        if let Some(room) = self.rooms.get(&room) {
            let msg = ClientServerMessage { msg_type, content };
            for (id, _) in &room.players {
                if skip_id.is_some() && id == skip_id.unwrap() {
                    continue;
                } else {
                    if let Some(addr) = self.sessions.get(&id) {
                        addr.do_send(msg.clone())
                    }
                }
            }
        }
    }

    pub fn send_user_message(&self, msg_type: MessageTypes, content: String, user_id: &Uuid) {
        if let Some(session) = self.sessions.get(&user_id) {
            let msg = ClientServerMessage { msg_type, content };
            session.do_send(msg);
        }
    }

    pub fn get_room_details(&self, room_id: Uuid) -> Option<&Room> {
        if let Some(room) = self.rooms.get(&room_id) {
            Some(room)
        } else {
            None
        }
    }

    pub fn get_player_details(&self, user_id: Uuid, room_id: Uuid) -> Option<&Player> {
        if let Some(room) = self.rooms.get(&room_id) {
            if let Some(player) = room.players.get(&user_id) {
                Some(player)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn join_room(&mut self, user_id: Uuid, room_id: Uuid, username: String) -> Player {
        let mut new_player = Player::default(String::new());
        if let Some(room) = self.rooms.get_mut(&room_id) {
            if let Some(player) = room.players.get_mut(&user_id) {
                player.active = true;
                new_player = player.clone();
            } else {
                new_player.username = username;
                room.players.insert(user_id, new_player.clone());
            }
        } else {
            self.rooms
                .insert(room_id, Room::new(user_id, room_id, username));
        }
        return new_player;
    }

    // pub fn start_game(&mut self, msg: StartGameMsg) {
    //     if let Some(room) = self.rooms.get_mut(&msg.room_id) {
    //         if room.owner == msg.user_id {
    //             room.status = GameStatus::STARTED;
    //             room.state = msg.state;
    //
    //             room.state.correct_word = "default".to_string();
    //             let player_to_draw = room.players.clone().into_keys().next();
    //             room.state.currently_drawing = player_to_draw;
    //             room.state.round_start_time = SystemTime::now()
    //                 .duration_since(UNIX_EPOCH)
    //                 .unwrap()
    //                 .as_millis();
    //
    //             let room_clone = room.clone();
    //             self.send_message(
    //                 &msg.room_id,
    //                 MessageTypes::Game(GameTypes::StartGame),
    //                 serde_json::to_string(&room_clone.state).unwrap(),
    //                 None,
    //             )
    //         }
    //     }
    // }

    pub fn start_game(&mut self, msg: StartGameMsg) {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            let data = room.start_game(msg.clone());
            if let Some(content) = data {
                self.send_message(
                    &msg.room_id,
                    MessageTypes::Game(GameTypes::NewTurn),
                    content,
                    None,
                )
            }
        }
    }

    pub fn validate_guess(&mut self, msg: ClientServerMessage, room_id: Uuid) {
        let data = serde_json::from_str::<UserGuessMsg>(&msg.content);
        if data.is_err() {
            return;
        }
        let data = data.unwrap();
        if let Some(room) = self.rooms.get_mut(&room_id) {
            let content = room.validate_guess(data);
            self.send_message(
                &room_id,
                MessageTypes::Game(GameTypes::GuessResult),
                serde_json::to_string(&content).unwrap(),
                None,
            )
        }
    }

    pub fn end_turn(&mut self, msg: ClientServerMessage, room_id: Uuid) {
        let data = serde_json::from_str::<EndGameMsg>(&msg.content);
        if data.is_err() {
            return;
        }
        let data = data.unwrap();
        if let Some(room) = self.rooms.get_mut(&room_id) {
            let (content, game_ended) = room.end_turn(data);
            if content.is_some() {
                let room_data = content.unwrap();
                if game_ended {
                    println!("send end game msg");
                    self.send_message(
                        &room_id,
                        MessageTypes::Game(GameTypes::EndGame),
                        serde_json::to_string(&room_data).unwrap_or(String::new()),
                        None,
                    );
                    self.rooms.remove(&room_data.room_id);
                } else {
                    self.send_message(
                        &room_id,
                        MessageTypes::Game(GameTypes::NewTurn),
                        serde_json::to_string(&room_data).unwrap_or(String::new()),
                        None,
                    )
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerMovement {
    enter: bool,
    user_id: Uuid,
    player: Player,
}

impl Handler<Connect> for Server {
    type Result = ();
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) {
        self.sessions.insert(msg.user_id, msg.addr);

        let player = self.join_room(msg.user_id, msg.room_id, msg.username);

        let info = PlayerMovement {
            enter: true,
            user_id: msg.user_id,
            player,
        };

        // notify all users in same room
        self.send_message(
            &msg.room_id,
            MessageTypes::Game(GameTypes::PlayerJoined),
            serde_json::to_string(&info).unwrap(),
            Some(&msg.user_id),
        );
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        if self.sessions.remove(&msg.user_id).is_some() {
            // self.rooms
            //     .get_mut(&msg.room_id)
            //     .unwrap()
            //     .players
            //     .remove(&msg.user_id);

            if let Some(room) = self.rooms.get_mut(&msg.room_id) {
                if room.players.len() > 1 {
                    if let Some(player) = room.players.get_mut(&msg.user_id) {
                        player.active = false;
                    }
                    // room.players.remove(&msg.user_id);
                } else {
                    let no_other_active_players = room
                        .players
                        .iter()
                        .find(|(_, player)| (*player).active)
                        .is_none();
                    if no_other_active_players {
                        println!("Close room");
                        self.rooms.remove(&msg.room_id);
                    }
                }
            }

            let info = PlayerMovement {
                enter: false,
                user_id: msg.user_id,
                player: Player::default(String::new()),
            };

            self.send_message(
                &msg.room_id,
                MessageTypes::Game(GameTypes::PlayerLeft),
                serde_json::to_string(&info).unwrap(),
                Some(&msg.user_id),
            )
        }
    }
}

impl Handler<UserMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: UserMessage, _ctx: &mut Self::Context) {
        if let Ok(data) = serde_json::from_str::<ClientServerMessage>(&msg.msg) {
            if data.msg_type == MessageTypes::Game(GameTypes::StartGame) {
                if let Ok(start_msg) = serde_json::from_str::<StartGameMsg>(&data.content) {
                    self.start_game(start_msg);
                }
            } else if data.msg_type == MessageTypes::Game(GameTypes::EndTurn) {
                println!("end turn msg received");
                self.end_turn(data, msg.room_id);
            } else if data.msg_type == MessageTypes::Game(GameTypes::Guess) {
                self.validate_guess(data, msg.room_id);
            } else {
                self.send_message(
                    &msg.room_id,
                    data.msg_type,
                    data.content,
                    Some(&msg.user_id),
                )
            }
        };
    }
}

impl Handler<RoomDetails> for Server {
    type Result = String;

    fn handle(&mut self, msg: RoomDetails, _ctx: &mut Self::Context) -> Self::Result {
        let details = self.get_room_details(msg.0);
        if details.is_some() {
            serde_json::to_string(details.unwrap()).unwrap_or(String::new())
        } else {
            String::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerDetails {
    user_id: Uuid,
    player: Player,
}

impl Handler<GetPlayerDetails> for Server {
    type Result = String;

    fn handle(&mut self, msg: GetPlayerDetails, _ctx: &mut Self::Context) -> Self::Result {
        let details = self.get_player_details(msg.user_id, msg.room_id);
        if details.is_some() {
            let res = PlayerDetails {
                user_id: msg.user_id,
                player: details.unwrap().clone(),
            };
            serde_json::to_string(&res).unwrap_or(String::new())
            // self.send_user_message(
            //     MessageTypes::Data(DataTypes::UserData),
            //     serde_json::to_string(&res).unwrap(),
            //     &msg.user_id,
            // )
        } else {
            String::new()
        }
    }
}
