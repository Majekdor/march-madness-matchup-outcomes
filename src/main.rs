mod pair;

use pair::Pair;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicI32, Ordering};

/// # Match-Up Combinations
/// **Author:** Kevin Barnes
///
/// **Description:** This program takes any number of game match-ups and tells you all possible
/// combinations of those match-ups.
///
/// **Note:** There are only two outcomes: win or lose. There are no ties.
///
/// **Example:** If you gave the program 2 match-ups there will be four possible combinations
/// of the outcomes of those match-ups. 3 match-ups would be 8 combinations and so on.
///
/// **Formula:** Number of possible outcomes equals 2 raised to the number of match-ups.
fn main() {
    println!("\nWelcome! This application will allow you to see all possible combinations of \
        any number of match-ups you enter.");
    println!("You will be prompted to enter home and away teams. It doesn't matter which team \
        you enter for each, this is just to distinguish between games.\n");

    let mut match_ups: Vec<Pair<String>> = vec![];

    let mut counter = 0;
    // Loop until the user is done entering match-ups
    loop {
        // Get the home team and trim the newline character
        stdout().write("Enter home team: ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut home_team = String::new();
        stdin().read_line(&mut home_team).unwrap();
        home_team = home_team.trim_end_matches("\n").parse().unwrap();

        // Get the away team and trim the newline character
        stdout().write("Enter away team: ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut away_team = String::new();
        stdin().read_line(&mut away_team).unwrap();
        away_team = away_team.trim_end_matches("\n").parse().unwrap();

        // Add the home team vs away team match-up to the match-ups vector
        match_ups.insert(counter, Pair::from(home_team, away_team));

        // Ask the user if they want to enter another match-up
        stdout().write("Enter another game? (yes/no) ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut cont_input = String::new();
        stdin().read_line(&mut cont_input).unwrap();
        cont_input = cont_input.trim_end_matches("\n").parse().unwrap();

        // Exit if the user doesn't want to enter another match-up
        if !cont_input.eq_ignore_ascii_case("yes") {
            println!();
            break;
        }
        counter += 1;
    }

    println!("Determining all possible combinations of winners...\n");
    recurse(
        &match_ups,
        0,
        &String::from("Winners:")
    );
    println!("\nTotal outcomes: {}", OUTCOMES_COUNTER.fetch_add(0, Ordering::Relaxed));
}

/// Counter for the number of outcomes of the match-ups.
static OUTCOMES_COUNTER: AtomicI32 = AtomicI32::new(0);

/// Recursively determine all possible combinations of winners from an array of match-ups.
fn recurse(values: &Vec<Pair<String>>, level: usize, winners: &String) {
    if level < values.len() {
        // This is either 1 or 2, which picks the first or second team from the pair to be the winner
        for i in 1..3 {
            recurse(
                values,
                level + 1,
                &(winners.to_owned() + "\n- " + &values.get(level).unwrap().get(i))
            );
        }
    } else {
        // Increment winner count and print this combination of winners
        OUTCOMES_COUNTER.fetch_add(1, Ordering::Relaxed);
        println!("{}", winners);
    }
}
