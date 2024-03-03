use serde_json;
use std::{fs::File, io::BufReader};
use serde::{Deserialize, Serialize};
use encoding::Encoding;
use chrono::{Local, TimeZone, Utc};
use crate::message_data::MessageData;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    sender_name: String,
    timestamp_ms: i64,
    content: Option<String>
}
#[derive(Debug, Deserialize, Serialize)]
struct MessageContainer {
    messages: Vec<Message>
}


fn encode_as_latin1(input: &str) -> Vec<u8> {
    // Latin-1 encoding
    let encoding = encoding::all::ISO_8859_1;

    // Create a raw encoder with a specified trap (EncoderTrap::Strict, EncoderTrap::Replace, etc.)
    let mut encoder = encoding.raw_encoder();

    // Encode the input string
    let mut output = Vec::new();
    encoder.raw_feed(input, &mut output);

    output
}

pub fn read_messages_mul(paths: Vec<&str>) -> Vec<MessageData> {
    let mut message_data: Vec<MessageData> = Vec::new();

    for path in paths.iter().rev() {
        message_data.append(& mut read_messages(path));
    }

    message_data
}

pub fn read_messages(path: &str) -> Vec<MessageData> {
    let mut message_data: Vec<MessageData> = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        match serde_json::from_reader::<_, MessageContainer>(reader) {
            Ok(data) => {
                for msg in data.messages {
                    match msg.content {
                        Some(text) => {
                            let body = String::from_utf8(encode_as_latin1(&text)).unwrap();

                            let timestamp_seconds = msg.timestamp_ms/1000;
                            let adjusted_utc_datetime = Utc.timestamp_opt(timestamp_seconds as i64, 0).unwrap();
                            let local_datetime = adjusted_utc_datetime.with_timezone(&Local);

                            message_data.push(
                            MessageData {
                                    date: local_datetime,
                                    body,
                                    is_from_me: false
                                }
                            );
                        },
                        None => continue
                    }
                }
            },
            Err(err) => println!("Error {}", err)
        }
    } else {
        println!("Error opening file");
    }

    message_data.reverse();

    message_data
}