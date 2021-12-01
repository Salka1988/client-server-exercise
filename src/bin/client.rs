use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use serde::Serialize;

use server_client::Currency;
use server_client::types::summary::CurrencyEnum;

use crate::user_input::get_input;

fn main() {

    loop {
        match TcpStream::connect("localhost:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                let input: CurrencyEnum = get_input("Please type something...");

                match input {
                    CurrencyEnum::NONE => {
                        // Or send from backend what is better
                        println!("No Currency Available");
                    }
                    _ => {
                        let serialized = serde_json::to_string(&input).unwrap();
                        stream.write(serialized.as_bytes()).unwrap();
                        println!("Sent input, awaiting reply...");

                        let mut data = [0; 1024];

                        match stream.read(&mut data) {
                            Ok(_) => {
                                let res = String::from(String::from_utf8_lossy(&data)
                                    .trim_matches(char::from(0)));
                                let currency_enum = serde_json::from_str(&res).
                                    unwrap_or(CurrencyEnum::NONE);
                                println!("{:?}", currency_enum);
                            },
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
    println!("Terminated.");
}

mod user_input {
    use std::io;

    use server_client::types::summary::CurrencyEnum;

    pub fn get_input(prompt: &str) -> CurrencyEnum {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        formatter(input)

    }


    pub fn formatter(input: String) -> CurrencyEnum {
        // println!("{:?}", input.trim().to_uppercase().as_str());
        match input.trim().to_uppercase().as_str() {
            "ROT" => { CurrencyEnum::ROT }
            "DOT" => { CurrencyEnum::DOT }
            _ => { CurrencyEnum::NONE }
        }
    }


}
