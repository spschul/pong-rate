use std::io;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;
use std::f64;

fn main() {
    // Read the files
    let players_file: String = env::args().nth(1).expect(
        "Requires arguments: path to players file, path to matches file");

    let matches_file: String = env::args().nth(2).expect(
        "Requires arguments: path to players file, path to matches file");
    
    let players_file = File::open(players_file).expect("Player file not found.");
    let matches_file = File::open(matches_file).expect("Matches file not found.");

    let players_file = io::BufReader::new(&players_file);
    let matches_file = io::BufReader::new(&matches_file);

    // Create Players hash map
    let mut players: HashMap<String, i32> = players_file.lines().map(|line| (line.unwrap().trim().to_string(), 1600)).collect();

    // Go through the matches in the file, and calculate the skill of each player
    for line in matches_file.lines() {
        // Parse the line
        let line = line.unwrap();
        let line_segmented: Vec<&str> = line.split(',').map(|x| x.trim()).collect();
        let skills = (*players.get(line_segmented[0]).unwrap(), *players.get(line_segmented[1]).unwrap());
        let scores: (i32, i32) = (line_segmented[2].parse().unwrap(), line_segmented[3].parse().unwrap());
        let timestamp: u64 = line_segmented[4].parse().unwrap();

        // calculate updated skills
        let new_skills = calc_updated_skill(skills, scores, timestamp);

        // update skills
        *players.get_mut(line_segmented[0]).unwrap() = new_skills.0;
        *players.get_mut(line_segmented[1]).unwrap() = new_skills.1;
    }

    // print out each player and their skill
    for (player, skill) in &players {
        println!("{}: {}", player, skill);
    }
}

// update skill using the Elo rating system
// reference: https://metinmediamath.wordpress.com/2013/11/27/how-to-calculate-the-elo-rating-including-example/ 
fn calc_updated_skill(skills: (i32, i32), scores: (i32, i32), timestamp: u64) -> (i32, i32) {
    let skills = (skills.0 as f64, skills.1 as f64);
    let rating = (10.0f64.powf(skills.0 / 400.0f64), 10.0f64.powf(skills.1 / 400.0f64));
    let total = rating.0 + rating.1;
    let expected_score = (rating.0 / total, rating.1 / total);
    let actual = if scores.0 > scores.1 {
        ( 1.0f64, 0.0f64 )
    } else {
        ( 0.0f64, 1.0f64 )
    };
    let k = 32.0;
    ((skills.0 + k * (actual.0 - expected_score.0)).round() as i32, (skills.1 + k * (actual.1 - expected_score.1)).round() as i32)
}
