// export interface IMessage {
//   username: string;
//   id: string;
//   content: string;
// }

export interface IMessageType {
  [key: string]: string;
}

export interface IServerMsg {
  msg_type: IMessageType;
  content: string;
}

export interface IGuessMsg {
  username: string;
  user_id: string;
  content: string;
  correct?: boolean;
}

export interface IDrawMsg {
  color: number;
  size: number;
  x: number;
  y: number;
  beginning: boolean;
  end: boolean;
}

export interface IGameMsg {
  drawing: string;
  scores: { [key: string]: number };
}

export interface IPlayerMovement {
  enter: boolean;
  user_id: string;
  player: IPlayer;
}

export interface IRoom {
  room_id: string;
  status: string;
  players: { [key: string]: IPlayer };
  owner: string;
  state: IGameState;
}

export interface IGameState {
  total_rounds: number;
  current_round: number;
  currently_drawing: number;
  title: string;
  correct_word: string;
  round_start_time: number;
}

export const MessageTypes = {
  Relay: ["Info", "Draw", "Text"],
  Game: ["StartGame", "GameState"],
  Data: ["UserData"],
};
export interface IPlayer {
  username: string;
  score: number;
  prev_score: number
  active: boolean;
}

export interface PlayerData {
  user_id: string;
  player: IPlayer;
}
