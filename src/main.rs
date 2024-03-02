mod imessage_fetch;

fn main() {
    let db_path = String::from("res/out");

    let x = imessage_fetch::read_messages(db_path, Some(12), 89);

    println!("Hello, world!");
}