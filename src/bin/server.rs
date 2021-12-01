use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::borrow::BorrowMut;
use server_client::Currency;
use server_client::types::summary::CurrencyEnum;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("localhost:3333").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {

            let mut data = [0 as u8; 50]; // using 50 byte buffer

            let n = match socket.read(&mut data).await {
                Ok(size) => {
                    // println!("{:?}", String::from_utf8_lossy(&data).trim_matches(char::from(0)));
                    let res = String::from(String::from_utf8_lossy(&data).trim_matches(char::from
                        (0)));

                    let deserialized: CurrencyEnum = serde_json::from_str(&res).unwrap_or(CurrencyEnum::NONE);

                    match deserialized {
                        CurrencyEnum::DOT => {
                            String::from(String::from_utf8_lossy(&data).trim_matches(char::from(0)))

                        }
                        CurrencyEnum::ROT => {
                            String::from(String::from_utf8_lossy(&data).trim_matches(char::from(0)))
                        }
                        _ => {
                            String::from(String::from_utf8_lossy(&data).trim_matches(char::from(0)))
                        }
                    }
                },
                Err(_) => {
                    // println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    String::from(String::from("Error occurred"))
                }
            };
            if let Err(e) = socket.write_all(&n.as_bytes()).await {

                println!("{}", n);
                println!("{}", n);
                println!("failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}

