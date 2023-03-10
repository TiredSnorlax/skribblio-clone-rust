use std::{
    collections::BTreeMap,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::messages::{
    ClientServerMessage, EndGameMsg, GuessResultMsg, StartGameMsg, UserGuessMsg,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameStatus {
    WAITING,
    STARTED,
    OVER,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub total_rounds: usize,
    pub current_round: usize,
    pub currently_drawing: usize,
    pub title: String,
    pub correct_word: String,
    pub round_start_time: u128,
}

impl GameState {
    pub fn default() -> GameState {
        GameState {
            total_rounds: 3,
            current_round: 1,
            currently_drawing: 0,
            title: "Default room".to_string(),
            correct_word: "default".to_string(),
            round_start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub username: String,
    pub score: usize,
    pub prev_score: usize,
    pub active: bool,
}

impl Player {
    pub fn default(username: String) -> Self {
        Player {
            username,
            score: 0,
            prev_score: 0,
            active: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub room_id: Uuid,
    pub status: GameStatus,
    pub players: BTreeMap<Uuid, Player>,
    pub owner: Uuid,
    pub state: GameState,
}

impl Room {
    pub fn new(owner: Uuid, room_id: Uuid, owner_username: String) -> Room {
        let mut players = BTreeMap::new();
        players.insert(owner, Player::default(owner_username));
        Room {
            room_id,
            status: GameStatus::WAITING,
            players,
            owner,
            state: GameState::default(),
        }
    }

    pub fn start_game(&mut self, msg: StartGameMsg) -> Option<String> {
        if self.owner == msg.user_id {
            self.status = GameStatus::STARTED;
            self.state = msg.state;

            self.state.correct_word = "default".to_string();
            self.state.round_start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let self_clone = self.clone();
            Some(serde_json::to_string(&self_clone).unwrap())
        } else {
            None
        }
    }

    pub fn validate_guess(&mut self, data: UserGuessMsg) -> Option<GuessResultMsg> {
        let mut content = GuessResultMsg {
            user_id: data.user_id,
            username: data.username,
            content: data.content.clone(),
            correct: false,
        };
        if self.state.correct_word == data.content.trim() {
            content.correct = true;
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let elapsed_time = current_time - self.state.round_start_time;

            if let Some(player) = self.players.get_mut(&data.user_id) {
                player.prev_score = player.score;
                player.score += ((60 - (elapsed_time / 1000)) * 100 / 60) as usize;
            }
            println!("{:?}", self);
        }
        Some(content)
    }

    pub fn end_turn(&mut self, data: EndGameMsg) -> (Option<Self>, bool) {
        let mut game_ended = false;
        if let Some(currently_drawing) = self.players.iter().nth(self.state.currently_drawing) {
            if currently_drawing.0 == &data.user_id {
                if self.state.currently_drawing + 1 < self.players.len() {
                    self.state.currently_drawing += 1;
                } else {
                    if self.state.current_round + 1 <= self.state.total_rounds {
                        self.state.current_round += 1;
                        self.state.currently_drawing = 0;
                        println!("CHANGE ROUND");
                    } else {
                        println!("END GAME");
                        game_ended = true;
                    }
                }
            }
        }
        (Some(self.clone()), game_ended)
    }
}
