use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::messages::{self, ClientServerMessage, MessageTypes, UserMessage};
use crate::server::Server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct UserSession {
    pub id: Uuid,
    pub username: String,
    pub hb: Instant,
    pub room: Uuid,
    pub addr: Addr<Server>,
}

impl UserSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(messages::Disconnect {
                    user_id: act.id,
                    room_id: act.room,
                });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    pub fn new(
        room: Uuid,
        addr: Addr<Server>,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> UserSession {
        let mut session = UserSession {
            id: Uuid::new_v4(),
            username: String::new(),
            room,
            hb: Instant::now(),
            addr,
        };
        if id.is_some() {
            session.id = id.unwrap();
        };

        if username.is_some() {
            session.username = username.unwrap();
        };

        session
    }
}

impl Actor for UserSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        let addr = ctx.address();

        self.addr
            .send(messages::Connect {
                addr: addr.recipient(),
                user_id: self.id,
                username: self.username.clone(),
                room_id: self.room,
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => (),
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(messages::Disconnect {
            room_id: self.room,
            user_id: self.id,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                if s.to_string() == "GET_ID".to_string() {
                    let res = ClientServerMessage {
                        msg_type: MessageTypes::Data(messages::DataTypes::UserID),
                        content: self.id.to_string(),
                    };

                    // self.addr.do_send(GetPlayerDetails {
                    //     user_id: self.id,
                    //     room_id: self.room,
                    // });
                    ctx.text(serde_json::to_string(&res).unwrap_or(String::from("{}")))
                } else {
                    self.addr.do_send(UserMessage {
                        user_id: self.id,
                        msg: s.to_string(),
                        room_id: self.room,
                    })
                }
            }

            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<ClientServerMessage> for UserSession {
    type Result = ();

    fn handle(&mut self, msg: ClientServerMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap_or(String::from("{}")));
    }
}
