use std::path::Path;
use chrono::{DateTime, Local};
use rusqlite;

pub struct MessageData {
    date: DateTime<Local>,
    body: String,
    is_from_me: bool
}

pub struct RawMessageDate {
    row_id: i32,
    date: i32,
    text: String,
    attributed_body: String,
    handle_id: i32,
    is_from_me: bool
}

pub fn read_messages(db_location: String, message_size:Option<i32>, handle_identifyer:i32) -> Vec<MessageData> {
    let mut data = Vec::new();

    let mut connection = rusqlite::Connection::open(db_location).unwrap();

    let query = r#"
        SELECT message.ROWID, message.date, message.text, message.attributedBody, message.handle_id, message.is_from_me
        FROM message
        LEFT JOIN handle ON message.handle_id = handle.ROWID
    "#;
    
    let mut stament = connection.prepare(query).unwrap();

    let messages = stament.query_map([], |row| {
        Ok( RawMessageDate {
            row_id: row.get(0)?,
            date: row.get(1)?,
            text: row.get(2)?,
            attributed_body: row.get(3)?,
            handle_id: row.get(4)?,
            is_from_me: row.get(5)?
        })
    });

    return data
}