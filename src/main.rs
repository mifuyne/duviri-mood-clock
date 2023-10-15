use std::env;

const START_TIME: u32 = 1696363200; // UNIX timestamp of the first mood
const MOODS: [&str; 5] = ["Fear", "Joy", "Anger", "Envy", "Sorrow"];
const SECONDS_PER_MOOD: u32 = 7200;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let (command, time_query, mood) = parse_arg(&args);

    if command == "query"
    {
        println!("QUERY");
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
        // TODO: write out the logic code for updating the seed time and mood.
        //       - mood should set the mood_offset value by finding the index value of `new_mood`
        //       - Set both the mood offset and start time (seed time?) as env values?
    }
    else
    {
        panic!("No valid command given!")
    }

}

fn parse_arg(args: &[String]) -> (&str, u32, Option<&String>) {
    let command = &args[1];
    let time = *&args[2].parse().expect("This cannot covert to a u32!");
    let mood = match args.len() {
        4 => Some(&args[3]),
        _ => None,
    };

    (command, time, mood)
}

fn which_mood(timestamp: u32) -> u32 {
    (timestamp - START_TIME) % 36000 
}