use std::{fs::File, io::Write};

mod imessage_fetch;
mod instagram_fetch;
mod attributed_text;
mod message_data;

const OUTPUT_DIR: &str = "out/";
const DB_ROW_LIMIT: Option<i32> = None;
const HANDLE_ID_IDENTIFYER: i32 = 89; // 😍😘👅👅👅👅

fn main() {
    let db_path = String::from("res/chat.db");

    if false {
        let msg = imessage_fetch::read_messages(db_path, DB_ROW_LIMIT, HANDLE_ID_IDENTIFYER);

        let mut i_message_file = File::create(OUTPUT_DIR.to_string()+"messages.csv").unwrap();

        // Save to file 
        for message in msg {
            writeln!(i_message_file, "{}", message).unwrap();
        }
    } else {
        instagram_fetch::read_messages("res/instagram/message_1.json");
    }
}