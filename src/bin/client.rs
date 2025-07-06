use std::net::Ipv4Addr;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key:       String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key:       String,
        val:       Bytes,
        responder: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect((Ipv4Addr::LOCALHOST, 6142)).await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, responder } => {
                    let response = client.get(&key).await;
                    let _ = responder.send(response);
                },
                Set {
                    key,
                    val,
                    responder,
                } => {
                    let response = client.set(&key, val).await;
                    let _ = responder.send(response);
                },
            }
        }
    });

    {
        // Spawn two tasks, one setting a value and other querying for key that was set.
        let tx2 = tx.clone();

        let a = tokio::spawn(async move {
            let (responder_tx, responder_rx) = oneshot::channel();
            let cmd = Command::Get {
                key:       "foo".to_string(),
                responder: responder_tx,
            };
            // Send the GET request
            tx.send(cmd).await.unwrap();

            // Await the response
            let response = responder_rx.await;
            println!("GOT = {response:?}");
        });

        let b = tokio::spawn(async move {
            let (responder_tx, responder_rx) = oneshot::channel();
            let cmd = Command::Set {
                key:       "foo".to_string(),
                val:       "bar".into(),
                responder: responder_tx,
            };
            // Send the SET request
            tx2.send(cmd).await.unwrap();

            // Await the response
            let response = responder_rx.await;
            println!("GOT = {response:?}");
        });

        a.await.unwrap();
        b.await.unwrap();
    }

    manager.await.unwrap();
}
