use crate::messages::Messages;

mod attributed_text;
mod msg_util;
// mod message_analysis;
mod messages;
mod message_analysis;

//
fn main() {
    let merged_msg = Messages::from_merge(
vec![
            Messages::from_instagram("res/adri_main_1.json", "mateo", "Adri Main"),
            Messages::from_instagram("res/adri_main_2.json", "mateo", "Adri Main"),
            Messages::from_instagram("res/adri_private_1.json", "mateo", "Adri Priv"),
            Messages::from_instagram("res/adri_private_2.json", "mateo", "Adri Priv"),
            Messages::from_instagram("res/chinese_dogs_1.json", "mateo", "Chinese Dog"),
            Messages::from_imessage_database("res/chat.db", None, 89, "iMessage")
        ]
    );

    merged_msg.save_to_csv("out/concat_messages.csv");

    // message_analysis::dates_freq(&merged_msg);
}