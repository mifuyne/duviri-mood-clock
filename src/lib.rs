use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json;
use wasm_bindgen::prelude::*;

const ENV_SEED_TIME: &'static str = env!("SEED_TIME", "SEED_TIME not set!");
const ENV_MOOD_OFFSET: &'static str = env!("MOOD_OFFSET", "MOOD_OFFSET not set!");
const ENV_SECONDS_PER_MOOD: &'static str = env!("SECONDS_PER_MOOD", "SECONDS_PER_MOOD not set!");
pub const MOODS: [&str; 5] = ["Fear", "Joy", "Anger", "Envy", "Sorrow"];

#[derive(Serialize)]
struct MoodItem {
    time: String,
    timestamp: i64,
    mood: String,
}

#[wasm_bindgen]
pub fn get_current_mood() -> String {
    let curr_time = Utc::now();
    let mood_now = format!(r#"{}"#, MOODS[which_mood(curr_time.timestamp())]);

    let curr_mood = MoodItem {
        time: curr_time.to_rfc3339(),
        timestamp: curr_time.timestamp(),
        mood: mood_now,
    };
    // TODO: Logic for determining current mood
    let serialized =
        serde_json::to_string(&curr_mood).expect("[ERROR] Current mood cannot be serialized!");

    format!("{}", serialized)
}

#[wasm_bindgen]
pub fn get_next_mood(datetime: &str, limit: i32) -> String {
    let next_timestamp = get_next_shift(datetime);
    let mut mood_list: Vec<MoodItem> = Vec::new();
    let seconds_per_mood = ENV_SECONDS_PER_MOOD
        .parse::<i64>()
        .expect("Cannot convert SECONDS_PER_MOOD environment variable!");

    for next in 0..limit {
        let mut this_time = next_timestamp;

        // if next is not the first item, then start adding seconds_per_mood to the previous item
        if next != 0 {
            this_time = mood_list[mood_list.len() - 1].timestamp + seconds_per_mood;
        }

        let this_mood = format!(r#"{}"#, MOODS[which_mood(this_time)]);

        mood_list.push(MoodItem {
            time: match DateTime::from_timestamp(this_time, 0) {
                Some(time) => time.to_rfc3339(),
                None => "0".to_string(),
            },
            timestamp: this_time,
            mood: this_mood,
        });
    }

    let serialized = serde_json::to_string(&mood_list).expect("Cannot serialize!");

    format!("{}", serialized)
}

// INTERNAL FUNCTIONS

// Convert to Epoch (Unix) time
fn get_epoch_time(datetime: String) -> String {
    // Is datetime in the proper format (%Y-%m-%dT%T%:z)
    match DateTime::parse_from_rfc3339(&datetime) {
        Ok(time) => return format!("{}", time.timestamp()),
        Err(e) => return format!("[ERROR] {}: {}", e, datetime),
    }
}

// Get mood based on timestamp provided
#[wasm_bindgen]
pub fn which_mood(timestamp: i64) -> usize {
    let mut start_time: i64 = 0;
    if let Ok(time) = DateTime::parse_from_rfc3339(ENV_SEED_TIME) {
        start_time = time.timestamp();
    }

    // TODO: Guard against start_time being a 0. Leverage Result.
    // Converting the ENV_SECONDS_PER_MOOD from &str to usize (to match MOODS.len() value type)
    let seconds_per_mood: usize = ENV_SECONDS_PER_MOOD
        .parse::<usize>()
        .expect("[ERROR]: Cannot parse ENV_SECONDS_PER_MOOD");

    // Get the time difference between now and SEED_TIME, within the 7200 seconds range
    let time_diff = ((timestamp - start_time) as usize) % (seconds_per_mood * MOODS.len());

    // Convert the time difference to match the index value of MOODS
    match time_diff {
        e if e < seconds_per_mood => 0,
        e if e < (seconds_per_mood * 2) => 1,
        e if e < (seconds_per_mood * 3) => 2,
        e if e < (seconds_per_mood * 4) => 3,
        e if e < (seconds_per_mood * 5) => 4,
        _ => 99,
    }
}

// Return the next time mood changes
fn get_next_shift(datetime: &str) -> i64 {
    let curr_time = datetime.to_owned().parse::<i64>().unwrap();

    curr_time + (7200 - (curr_time % 7200))
}
