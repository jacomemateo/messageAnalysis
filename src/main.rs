use std::collections::HashMap;
use crate::messages::Messages;
use rand::Rng;
use std::io;
mod attributed_text;
mod msg_util;
mod messages;
use chrono::{Duration, NaiveDate};

fn datesFreq(messages: &Messages) {
    let mut dates_freq: HashMap<NaiveDate, i32> = HashMap::new();

    for message in &messages.message_vec {
        let date = message.date.date_naive();
        *dates_freq.entry(date).or_insert(0) += 1;
    }

    let mut sorted_dates: Vec<_> = dates_freq.iter().collect();
    sorted_dates.sort_by_key(|&(date, _count)| date);

    let first_date = sorted_dates.first().map_or(NaiveDate::from_ymd_opt(2000, 1, 1), |&(date, _count) | Some(*date)).unwrap();
    let last_date = sorted_dates.last().map_or(NaiveDate::from_ymd_opt(2000, 1, 1), |&(date, _count)| Some(*date)).unwrap();

    let duration = last_date.signed_duration_since(first_date).num_days();
    let all_dates: Vec<_> = (0..=duration).map(|day| first_date + Duration::days(day)).collect();

    let new: HashMap<NaiveDate, i32> = all_dates.iter().cloned().map(|date| (date, *dates_freq.get(&date).unwrap_or(&0))).collect();

    let mut sorted_dates: Vec<_> = new.iter().collect();
    sorted_dates.sort_by_key(|&(date, _count)| date);

    for (date, count) in sorted_dates {
        println!("{}, {}", date, count);
    }
}

fn getRandomMessage(message: &Messages) {
    let mut rng = rand::thread_rng();
    let mut input = String::new();

    while input != "q" {
        let mut index = rng.gen_range(0..message.message_size());
        println!("From {}\t{}", if message.message_vec[index].is_from_me {"mateo"} else {"her"}, message.message_vec[index].body);
        io::stdin().read_line(&mut input).expect("Failed to read line");
    }
}

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
    println!("Messages successfully saves to file!");

    let mut input = String::new();
    println!("What would you like to do?");
    println!("1. Display number of messages sent per day.");
    println!("2. Display random messages (press enter to see next message, 'q' to exit).");

    io::stdin().read_line(& mut input).expect("Failed to read line");

    match input.trim().parse::<i32>().unwrap() {
        1 => datesFreq(&merged_msg),
        2 => getRandomMessage(&merged_msg),
        _ => println!("Goodbye.")
    }
}