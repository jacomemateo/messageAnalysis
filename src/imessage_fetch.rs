use chrono::{DateTime, FixedOffset};
use rusqlite;
use crate::attributed_text;
use crate::message_data::MessageData;

struct BannedWords {
    phrases: &'static [&'static str],
    reactions: &'static [&'static str],
}
pub struct RawMessageData {
    date: i64,
    text:  Option<String>,
    attributed_body: Option<Vec<u8>>,
    is_from_me: bool
}

const SECOND: i64 = 1_000_000_000;

fn apple_date_to_datetime(date: i64) -> DateTime<FixedOffset> {
    let date_str = "2001-01-01 00:00:00 -05:00";
    let jan_2001 = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z").unwrap();

    let new_date = jan_2001 + chrono::Duration::seconds(date / SECOND);

    new_date
}

pub fn read_messages(db_location: String, message_size:Option<i32>, handle_identifyer:i32) -> Vec<MessageData> {
    const BANNED: BannedWords = BannedWords {
        phrases: &[
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
        reactions: &["Loved “", "Laughed at “", "Questioned “", "Liked “", "Emphasized “", "Disliked “"],
    };

    let mut message_data: Vec<MessageData> = Vec::new();

    let connection = rusqlite::Connection::open(db_location).unwrap();

    let mut query = r#"
        SELECT message.ROWID, message.date, message.text, message.attributedBody, message.handle_id, message.is_from_me
        FROM message
        LEFT JOIN handle ON message.handle_id = handle.ROWID
    "#.to_string();
    
    query += &format!("WHERE message.handle_id = {}", handle_identifyer);

    if let Some(message_size) = message_size {
        query += &format!(" ORDER BY message.date DESC LIMIT {}", message_size);
    }

    let mut stament = connection.prepare(query.as_str()).unwrap();

    let messages = stament.query_map([], |row| {
        Ok( RawMessageData {
            date: row.get(1)?,
            text: row.get(2)?,
            attributed_body: row.get(3)?,
            is_from_me: row.get(5)?
        })
    }).unwrap();

    'individual_word: for message in messages {
        let msg = message.unwrap();

        let mut text = String::new();

        if let Some(body) = msg.text {
            text = body;
        } else if let Some(blob) = msg.attributed_body {
            text = attributed_text::parse(blob);
        }
    
        if text.len() == 0 {
            continue 'individual_word;
        }

        for char in text.chars() {
            let code_point = char as u32;
            if code_point >= 0xFFF0 && code_point <= 0xFFFF {
                continue 'individual_word;
            }
        }

        for phrase in BANNED.phrases {
            if text == phrase.to_owned() {
                continue 'individual_word;
            }
        }

        for phrase in BANNED.reactions {
            if text.contains(phrase) {
                continue 'individual_word;
            }
        }

        let date = apple_date_to_datetime(msg.date);
            
        message_data.push(
            MessageData {
                date,
                body: text,
                is_from_me: msg.is_from_me
            }
        );
    }


    return message_data
}