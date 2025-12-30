use mini_redis::{Connection, Frame, Result};
use redis::address;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(address(6379)).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use std::collections::HashMap;

    use mini_redis::Command::{self, Get, Set};

    let mut db = HashMap::new();
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        println!("GOT: {frame:?}");

        let response = match Command::from_frame(frame).unwrap() {
            Set(prop) => {
                db.insert(prop.key().to_owned(), prop.value().to_vec());
                Frame::Simple("OK".to_owned())
            }
            Get(prop) => {
                if let Some(value) = db.get(prop.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            command => panic!("unimplemented {command:?}"),
        };

        conn.write_frame(&response).await.unwrap();
    }
}
