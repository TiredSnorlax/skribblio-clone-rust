use std::str::FromStr;

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    get,
    http::header,
    post,
    web::{self, Path},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use server::Server;
use uuid::Uuid;

use crate::{
    messages::{GetPlayerDetails, RoomDetails},
    session::UserSession,
};

mod messages;
mod room;
mod server;
mod session;

#[derive(Serialize)]
struct ErrorResponse {
    msg: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectQuery {
    session: String,
    username: String,
}

#[get("/ws/{room_id}")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    room_id: Path<String>,
    server: web::Data<Addr<Server>>,
    query: web::Query<ConnectQuery>,
) -> Result<HttpResponse, Error> {
    let query = &query.into_inner();
    let mut session = None;
    let mut username = None;

    let room_id = Uuid::from_str(&room_id);

    if let Ok(temp) = Uuid::from_str(&query.session) {
        session = Some(temp);
    }

    if query.username.len() > 0 {
        username = Some(query.username.clone());
    };

    if room_id.is_ok() {
        ws::start(
            UserSession::new(
                room_id.unwrap(),
                server.get_ref().clone(),
                session,
                username,
            ),
            &req,
            stream,
        )
    } else {
        Ok(HttpResponse::BadRequest().body("Invalid room id"))
    }
}

#[post("/details/{room_id}")]
async fn get_room_details(
    room_id: Path<String>,
    server: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let room_id = Uuid::from_str(&room_id);
    if room_id.is_ok() {
        let mut details = String::new();
        let test = server.get_ref().send(RoomDetails(room_id.unwrap())).await;

        if test.is_ok() {
            details = test.unwrap();
        }

        Ok(HttpResponse::Ok().body(details))
    } else {
        Ok(HttpResponse::BadRequest().body("Invalid room id"))
    }
}

#[derive(Debug, Deserialize)]
struct PlayerDetailsQuery {
    user_id: String,
    room_id: String,
}

#[post("/room/{room_id}/player/{user_id}")]
async fn get_player_details(
    query: Path<PlayerDetailsQuery>,
    server: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();
    let room_id = Uuid::from_str(&query.room_id);
    let user_id = Uuid::from_str(&query.user_id);
    if room_id.is_ok() && user_id.is_ok() {
        let details = server
            .get_ref()
            .send(GetPlayerDetails {
                user_id: user_id.unwrap(),
                room_id: room_id.unwrap(),
            })
            .await;

        Ok(HttpResponse::Ok().body(details.unwrap_or(String::new())))
    } else {
        Ok(HttpResponse::BadRequest().body("Invalid room id"))
    }
}

#[post("/room/new")]
async fn get_new_room_id() -> impl Responder {
    HttpResponse::Ok().body(Uuid::new_v4().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(index)
            .service(get_room_details)
            .service(get_new_room_id)
            .service(get_player_details)
            .app_data(web::Data::new(server.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
