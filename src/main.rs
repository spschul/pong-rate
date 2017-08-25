use std::io;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::f64;

fn main() {
    let players_file: String = env::args().nth(1).expect(
        "Requires arguments: path to players file, path to matches file");

    let matches_file: String = env::args().nth(2).expect(
        "Requires arguments: path to players file, path to matches file");
    
    let players_file = File::open(players_file).expect("Player file not found.");
    let matches_file = File::open(matches_file).expect("Matches file not found.");

    let mut players_file = io::BufReader::new(&players_file);
    let mut matches_file = io::BufReader::new(&matches_file);

    let mut players: HashMap<String, i32> = players_file.lines().map(|line| (line.unwrap().trim().to_string(), 100)).collect();

    println!("{:?}", players);

    for line in matches_file.lines() {
        let line = line.unwrap();
        let line_segmented: Vec<&str> = line.split(',').map(|x| x.trim()).collect();
        let player1_skill = *players.get(line_segmented[0]).unwrap();
        let player2_skill = *players.get(line_segmented[1]).unwrap();
        let player1_score: i32 = line_segmented[2].parse().unwrap();
        let player2_score: i32 = line_segmented[3].parse().unwrap();
        let timestamp: u64 = line_segmented[4].parse().unwrap();

        let (player1_new_skill, player2_new_skill) = 
            calc_updated_skill(player1_skill, player2_skill, player1_score,
                               player2_score, timestamp);

        {
            let old_skill = players.get_mut(line_segmented[0]).unwrap();
            *old_skill = player1_new_skill;
        }
        {
            let old_skill = players.get_mut(line_segmented[1]).unwrap();
            *old_skill = player2_new_skill;
        }
    }

    for (player, skill) in &players {
        println!("{}: {}", player, skill);
    }

}

fn calc_updated_skill(player1_skill: i32, player2_skill: i32, player1_score: i32, player2_score: i32, timestamp: u64) -> (i32, i32) {
    let skills = (player1_skill as f64, player2_skill as f64);
    let rating = (10.0f64.powf(skills.0 / 400.0f64), 10.0f64.powf(skills.1 / 400.0f64));
    let total = skills.0 + skills.1;
    let expected_score = (skills.0 / total, skills.1 / total);
    let actual = if player1_score > player2_score {
        ( 1.0f64, 0.0f64 )
    } else {
        ( 0.0f64, 1.0f64 )
    };
    let K = 32.0;
    ((skills.0 + K * (actual.0 - expected_score.0)).round() as i32, (skills.1 + K * (actual.1 - expected_score.1)).round() as i32)
}

#[derive(Debug)]
struct Player {
    name: String,
    elo: i32,
}
