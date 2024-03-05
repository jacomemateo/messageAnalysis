use std::collections::HashMap;
use chrono::{Duration, NaiveDate};
use crate::messages::Messages;

pub fn dates_freq(messages: &Messages) {
    let mut dates_freq: HashMap<NaiveDate, i32> = HashMap::new();
    // instagram, i-message

    for message in &messages.message_vec {
        let date= message.date.date_naive();

        if !dates_freq.contains_key(&date) {
            dates_freq.insert(date, 0);
        } else {
            dates_freq.insert(date, dates_freq.get(&date).unwrap().clone() + 1 );
        }
    }

    let mut sorted_dates: Vec<_> = dates_freq.iter().collect();
    sorted_dates.sort_by(|a, b| a.0.cmp(b.0));

    let first_date = sorted_dates.get(0).unwrap().0.clone();
    let last_date = sorted_dates.last().unwrap().0.clone();


    let duration = last_date.signed_duration_since(first_date).num_days();

    let mut all_dates = vec![first_date];

    for day in 1..duration {
        all_dates.push(first_date + Duration::days(day));
    }
    all_dates.push(last_date);

    let mut new: HashMap<NaiveDate,i32> = HashMap::new();

    for date in all_dates {
        if !dates_freq.contains_key(&date) {
            new.insert(date, 0);
        } else {
            new.insert(date, dates_freq.get(&date).unwrap().clone() );
        }
    }

    let mut sorted_dates: Vec<_> = new.iter().collect();
    sorted_dates.sort_by(|a, b| a.0.cmp(b.0));

    for date in sorted_dates {
        println!("{}, {}", date.0, date.1)
    }
}