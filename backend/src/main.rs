#[macro_use]
extern crate rocket;

use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use common::{ChatMessage, WebSocketMessage};
use rocket::{
    State,
    futures::{SinkExt, StreamExt, stream::SplitSink},
    tokio::sync::Mutex,
};
use rocket_ws::{Channel, Message, WebSocket, stream::DuplexStream};
use serde_json::json;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, ChatRoomConnection>>,
}

struct ChatRoomConnection {
    username: String,
    sink: SplitSink<DuplexStream, Message>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut conns = self.connections.lock().await;
        let connection = ChatRoomConnection {
            username: format!("User #{}", id),
            sink,
        };
        conns.insert(id, connection);
    }

    pub async fn send_username(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        let connection = conns.get_mut(&id).unwrap();

        let websocket_message = WebSocketMessage {
            message_type: common::WebSocketMessageType::UsernameChange,
            message: None,
            username: Some(connection.username.clone()),
            users: None,
        };

        let _ = connection
            .sink
            .send(Message::Text(json!(websocket_message).to_string()))
            .await;
    }

    pub async fn change_username(&self, new_username: String, id: usize) {
        let mut conns = self.connections.lock().await;
        let connection = conns.get_mut(&id).unwrap();
        connection.username = new_username;
    }

    pub async fn broadcast_message(&self, message: ChatMessage) {
        let mut conns = self.connections.lock().await;
        let websocket_message = WebSocketMessage {
            message_type: common::WebSocketMessageType::NewMessage,
            message: Some(message),
            username: None,
            users: None,
        };

        for (_id, connection) in conns.iter_mut() {
            let _ = connection
                .sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_user_list(&self) {
        let mut conns = self.connections.lock().await;
        let mut user_list = vec![];

        for (_, connection) in conns.iter() {
            user_list.push(connection.username.clone());
        }

        let websocket_message = WebSocketMessage {
            message_type: common::WebSocketMessageType::UsersList,
            message: None,
            username: None,
            users: Some(user_list),
        };

        for (_, connection) in conns.iter_mut() {
            let _ = connection
                .sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn remove(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        conns.remove(&id);
    }
}

async fn handle_incoming_message(
    message_content: Message,
    state: &State<ChatRoom>,
    connection_id: usize,
) {
    match message_content {
        Message::Text(json) => {
            if let Ok(websocket_message) = serde_json::from_str::<WebSocketMessage>(&json) {
                match websocket_message.message_type {
                    common::WebSocketMessageType::NewMessage => {
                        if let Some(ws_msg) = websocket_message.message {
                            state.broadcast_message(ws_msg).await;
                        }
                    }
                    common::WebSocketMessageType::UsernameChange => {
                        if let Some(ws_username) = websocket_message.username {
                            state.change_username(ws_username, connection_id).await;
                            state.send_username(connection_id).await;
                            state.broadcast_user_list().await;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

#[get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            state.add(user_id, ws_sink).await;
            state.broadcast_user_list().await;
            state.send_username(user_id).await;

            while let Some(message) = ws_stream.next().await {
                if let Ok(message_content) = message {
                    handle_incoming_message(message_content, state, user_id).await;
                }
            }

            state.remove(user_id).await;
            state.broadcast_user_list().await;

            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![chat])
        .manage(ChatRoom::default())
        .launch()
        .await;
}
