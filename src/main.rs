use std::env;
use chrono::{Local, DateTime};

// GLOBALS
const MOODS: [&str; 5] = ["Fear", "Joy", "Anger", "Envy", "Sorrow"];

// MOOD_OFFSET should be adjusted via "update". How many steps away is the new seed mood from MOODS[0] (Fear)?
// const MOOD_OFFSET: &str = match option_env!("MOOD_OFFSET") {
//     Some(val) => val,
//     None => "0",
// };

const SEED_TIME: &'static str = env!("SEED_TIME", "SEED_TIME not set!");
// const SEED_TIME: &str = match env!("SEED_TIME") {
//     Some(datetime) => datetime,
//     None => "2023-10-03T16:00:00-04:00", // the original date-time when this app's concept conceived
// };
const SECONDS_PER_MOOD: i64 = 7200;

// ------------------------

fn main() {
    // Env Var check
    // DO NOT COMPILE if the required env vars are empty or improperly set!
    if SEED_TIME.is_empty() {
        panic!("SEED_TIME is blank!");
    }
    // Checking that SEED_TIME was set properly
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let (command, time_query, mood) = parse_arg(&args);

    // let mut unix_time: i64 = 0;

    // // Parsing SEED_TIME as epoch time
    // if let Ok(time) = DateTime::parse_from_rfc3339(SEED_TIME) {
    //     unix_time = time.timestamp();
    // }

    // Command Responses
    if command == "query"
    {
        println!("QUERY");
        dbg!(SEED_TIME);
        let current_mood: usize = match which_mood(time_query) {
            e if e < SECONDS_PER_MOOD => 0,
            e if e < (SECONDS_PER_MOOD * 2) => 1,
            e if e < (SECONDS_PER_MOOD * 3) => 2,
            e if e < (SECONDS_PER_MOOD * 4) => 3,
            e if e < (SECONDS_PER_MOOD * 5) => 4,
            _ => 99,
         };
     
         dbg!(MOODS[current_mood]);
    }
    else if command == "update"
    {
        let Some(new_mood) = mood else
        {
            panic!("Need a mood!");
            // return;
        };
        println!("This is an update attempt! Updating seed with {} and {}", time_query, new_mood);
        /*
        TODO: write out the logic code for updating the seed time and mood.
              - if time_query is invalid (probably should use the Result<i64, ParseError> type),
                then this should print an error message (or panic)
              - mood should set the mood_offset value by finding the index value of `new_mood`
              - Set both the mood offset and start time (seed time?) as env values?
            
            - OCT 18: is an update feature appropriate if the seed data is provided at compile time via env. var.?
        */
    }
    else
    {
        panic!("No valid command given!")
    }
}

// Parsing arguments from command line
//  (Temporary method of input, adjust for web form input when ready)
fn parse_arg(args: &[String]) -> (&str, i64, Option<&String>) {
    let command = &args[1];
    let mut time_query: i64 = 0;
    // let time = *&args[2].parse().expect("This cannot covert to a i64!");
    let mood = match args.len() {
        4 => Some(&args[3]),
        _ => None,
    };

    // RFC3339 format expected: %Y-%m-%dT%T%:z
    // Example: date "+%Y-%m-%dT%T%:z"
    // 1. Check if query time is actually the keyword "now"
    match *&args[2] == "now" {
        true => time_query = Local::now().timestamp(),
        false => {
            match *&args[2].parse()
            {
                // 2. Check if the query time is already in epoch time
                Ok(time) => {
                    dbg!(time);
                    time_query = time;
                },
                Err(_) => {
                    println!("This is not in Unix timestamp format! Attempting conversion...");
                    // Query time is not in epoch time, is it in RFC3339 format (%Y-%m-%dT%T%:z)?
                    match DateTime::parse_from_rfc3339(&args[2]) {
                        Ok(time) => time_query = time.timestamp(),
                        Err(e) => println!("[ERROR] {}: {}", e, &args[2]),
                    }
                },
            }
        }
    }


    (command, time_query, mood)
}

fn which_mood(timestamp: i64) -> i64 {
    let mut start_time: i64 = 0;
    if let Ok(time) = DateTime::parse_from_rfc3339(SEED_TIME) {
        start_time = time.timestamp();
    }

    (timestamp - start_time) % 36000
}