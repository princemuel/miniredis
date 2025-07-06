use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 6142))
        .await
        .unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

type Database = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: Database) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();
                db.get(cmd.key())
                    .map_or_else(|| Frame::Null, |value| Frame::Bulk(value.clone()))
            },
            cmd => unimplemented!("{cmd:?}"),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
