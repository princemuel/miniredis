use std::net::Ipv4Addr;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 6142)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            loop {
                match socket.read(&mut buffer).await {
                    // Return value of `Ok(0)` signifies that the remote has closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buffer[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    },
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    },
                }
            }
        });
    }
}
