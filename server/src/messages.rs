use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::room::GameState;

// #[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
// pub enum MessageTypes {
//     Info,
//     Draw,
//     Text,
//     Game,
//     StartGame,
//     GameState,
//     PlayerJoined,
//     PlayerLeft,
//     UserData,
// }

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageTypes {
    Relay(RelayTypes),
    Game(GameTypes),
    Data(DataTypes),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum RelayTypes {
    Info,
    Draw,
    Text,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum GameTypes {
    StartGame,
    GameState,
    PlayerJoined,
    PlayerLeft,
    Guess,
    GuessResult,
    EndTurn,
    NewTurn,
    EndGame,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum DataTypes {
    UserID,
}

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct ClientServerMessage {
    pub msg_type: MessageTypes,
    pub content: String,
}

// #[derive(Message, Serialize, Deserialize, Debug, Clone)]
// #[rtype(result = "()")]
// pub struct ClientServerMessage {
//     pub msg_type: MessageTypes,
//     pub content: String,
// }

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<ClientServerMessage>,
    pub user_id: Uuid,
    pub username: String,
    pub room_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Message, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub user_id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct UserGuessMsg {
    pub user_id: Uuid,
    pub username: String,
    pub content: String,
}

#[derive(Message, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct GuessResultMsg {
    pub user_id: Uuid,
    pub username: String,
    pub content: String,
    pub correct: bool,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct UserMessageData {
    pub msg_type: String,
    pub content: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(String)]
pub struct RoomDetails(pub Uuid);

#[derive(Message, Serialize, Deserialize)]
#[rtype(String)]
pub struct GetPlayerDetails {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct StartGameMsg {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub state: GameState,
}

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct EndGameMsg {
    pub user_id: Uuid,
    pub room_id: Uuid,
}
