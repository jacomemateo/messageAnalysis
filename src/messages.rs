use serde_json;
use std::{fs::File, io::BufReader, io::Write};
use encoding::Encoding;
use chrono::{DateTime, Local, Duration, Utc, TimeZone};
use crate::attributed_text;
use crate::msg_util::*;

/// Used for dividing nanoseconds from IMessage to seconds
const SECOND: i64 = 1_000_000_000;

const INSTAGRAM_BANNED: BannedWords = BannedWords {
    match_whole_word: &[""],
    match_in_word: &[
        "sent an attachment.",
        " to your message",
    ]
};

const I_MESSAGE_BANNED: BannedWords = BannedWords {
    match_whole_word: &[
        "Cup Pong",
        "20 Questions",
        "Mancala",
        "Archery",
        "8 Ball+",
        "8 Ball",
        "Basketball",
        "Archery",
        "Darts",
        "Mini Golf",
        "Knockout",
        "Word Hunt",
        " "
    ],
    match_in_word: &["Loved “", "Laughed at “", "Questioned “", "Liked “", "Emphasized “", "Disliked “"],
};

/// Vector of *MessageData*
pub struct Messages {
    pub message_vec: Vec<MessageData>
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            message_vec: Vec::new()
        }
    }

    pub fn save_to_csv(&self, path: &str) {
        let mut output_file = File::create(path).unwrap();
        writeln!(output_file, "{}", "Source, from_me, date, text").unwrap();

        for message in &self.message_vec {
            writeln!(output_file, "{}", message).unwrap();
        }
    }

    /// Concatenates multiple *Messages* objects into one and then sorts by date, descending
    pub fn from_merge(other_msg: Vec<Messages>) -> Messages {
        let mut merged: Vec<MessageData> = Vec::new();

        for database in other_msg {
            merged.extend(database.message_vec);
        }

        merged.sort_by(|a, b| a.date.cmp(&b.date));

        Messages { message_vec: merged }
    }

    /// Due to how facebook encodes the instagram messages we first have
    /// to encode the message as latin and then decode it as utf-8
    /// to get the correct output
    fn decode_instagram_text(input: &str) -> String {
        // Latin-1 encoding
        let encoding = encoding::all::ISO_8859_1;
        let mut encoder = encoding.raw_encoder();

        // Encode the input string
        let mut output = Vec::new();
        encoder.raw_feed(input, &mut output);

        String::from_utf8(output).unwrap()
    }

    pub fn from_instagram(path: &str, my_name: &str, message_source: &str) -> Messages {
        let mut message_vec: Vec<MessageData> = Vec::new();

        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("Error: {}", err)
        };

        let reader = BufReader::new(file);

        let deserialized_data = match serde_json::from_reader::<_, serde_json::Value>(reader) {
            Ok(data) => data,
            Err(err) => panic!("Error: {}", err)
        };

        for instagram_message in deserialized_data["messages"].as_array().unwrap() {
            let text = &instagram_message["content"].to_string();

            // Fix instagram message body
            let body = Self::decode_instagram_text(&text);

            if INSTAGRAM_BANNED.invalid(&body) { continue; }

            let timestamp_seconds = instagram_message["timestamp_ms"].as_i64().unwrap() / 1000;
            let adjusted_utc_datetime = Utc.timestamp_opt(timestamp_seconds, 0).unwrap();
            let local_datetime = adjusted_utc_datetime.with_timezone(&Local);

            let is_from_me = if my_name == instagram_message["sender_name"] { true } else { false };

            message_vec.push(
        MessageData {
                message_source: message_source.to_string(),
                date: local_datetime,
                body,
                is_from_me
            });
        }

        message_vec.reverse();
        Messages {message_vec}
    }

    fn apple_date_to_datetime(date: i64) -> DateTime<Local> {
        let jan_2001 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
        let adjusted_utc_datetime = jan_2001 + Duration::seconds(date / SECOND);

        let local_datetime = adjusted_utc_datetime.with_timezone(&Local);

        local_datetime
    }

    fn invalid_imessage_body(text: &String) -> bool {
        if text.len() == 0 {
            return true;
        }

        for char in text.chars() {
            let code_point = char as u32;
            if code_point >= 0xFFF0 && code_point <= 0xFFFF {
                return true;
            }
        }

        if I_MESSAGE_BANNED.invalid(&text) {
            return true;
        }

        return false
    }

    pub fn from_imessage_database(db_location: &str, message_size:Option<i32>, handle_identifier:i32, message_source: &str) -> Messages {
        let mut message_vec: Vec<MessageData> = Vec::new();

        let connection = match rusqlite::Connection::open(db_location) {
            Ok(connection) => connection,
            Err(err) => panic!("Error {}", err)
        };

        let mut query = r#"
        SELECT message.ROWID, message.date, message.text, message.attributedBody, message.handle_id, message.is_from_me
        FROM message
        LEFT JOIN handle ON message.handle_id = handle.ROWID
        "#.to_string();

        query += &format!("WHERE message.handle_id = {}", handle_identifier);

        // If message size is none query the whole database
        if let Some(message_size) = message_size {
            query += &format!(" ORDER BY message.date DESC LIMIT {}", message_size);
        }

        let mut statement = connection.prepare(query.as_str()).unwrap();

        let mut messages = statement.query([]).unwrap();

        while let Some(msg) = messages.next().unwrap() {
            let mut text = String::new();

            if let Some(body) = msg.get(2).unwrap() {
                text = body;
            } else if let Some(blob) = msg.get(3).unwrap() {
                text = attributed_text::parse(blob);
            }

            if Self::invalid_imessage_body(&text) {
                continue;
            }

            let date = Self::apple_date_to_datetime(msg.get(1).unwrap());

            message_vec.push(
                MessageData {
                    message_source: message_source.to_string(),
                    date,
                    body: text,
                    is_from_me: msg.get(5).unwrap()
                }
            );
        }

        Messages { message_vec }
    }
}