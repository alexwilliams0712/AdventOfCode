use std::fs;
use std::str::FromStr;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[derive(Debug, PartialEq)]
enum TheirHand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for TheirHand {
    type Err = ();

    fn from_str(input: &str) -> Result<TheirHand, Self::Err> {
        match input {
            "A" => Ok(TheirHand::Rock),
            "B" => Ok(TheirHand::Paper),
            "C" => Ok(TheirHand::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum YourHand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for YourHand {
    type Err = ();

    fn from_str(input: &str) -> Result<YourHand, Self::Err> {
        match input {
            "X" => Ok(YourHand::Rock),
            "Y" => Ok(YourHand::Paper),
            "Z" => Ok(YourHand::Scissors),
            _ => Err(()),
        }
    }
}

fn calculate_round_score(round: &str) -> u32 {
    let them = TheirHand::from_str(round.chars().nth(0).unwrap().to_string().as_str())
        .expect("Invalid value");

    let you = YourHand::from_str(round.chars().nth(2).unwrap().to_string().as_str())
        .expect("Invalid value");

    let mut score: u32 = match you {
        YourHand::Rock => 1,
        YourHand::Paper => 2,
        YourHand::Scissors => 3,
    };

    if ((you == YourHand::Rock) && (them == TheirHand::Scissors))
        || ((you == YourHand::Paper) && (them == TheirHand::Rock))
        || ((you == YourHand::Scissors) && (them == TheirHand::Paper))
    {
        score += 6
    } else if format!("{:?}", you) == format!("{:?}", them) {
        score += 3
    }

    score
}

#[derive(Debug, PartialEq)]
enum DesiredResult {
    WIN,
    LOSE,
    DRAW,
}

impl FromStr for DesiredResult {
    type Err = ();

    fn from_str(input: &str) -> Result<DesiredResult, Self::Err> {
        match input {
            "X" => Ok(DesiredResult::LOSE),
            "Y" => Ok(DesiredResult::DRAW),
            "Z" => Ok(DesiredResult::WIN),
            _ => Err(()),
        }
    }
}

fn calculate_correct_round_score(round: &str) -> u32 {
    let them = TheirHand::from_str(round.chars().nth(0).unwrap().to_string().as_str())
        .expect("Invalid value");

    let result = DesiredResult::from_str(round.chars().nth(2).unwrap().to_string().as_str())
        .expect("Invalid value");

    let mut score: u32 = match result {
        DesiredResult::WIN => 6,
        DesiredResult::DRAW => 3,
        DesiredResult::LOSE => 0,
    };
    let mut hand_score: u32 = 0;

    if result == DesiredResult::DRAW {
        hand_score = match them {
            TheirHand::Rock => 1,
            TheirHand::Paper => 2,
            TheirHand::Scissors => 3,
        }
    } else if result == DesiredResult::WIN {
        hand_score = match them {
            TheirHand::Rock => 2,
            TheirHand::Paper => 3,
            TheirHand::Scissors => 1,
        }
    } else if result == DesiredResult::LOSE {
        hand_score = match them {
            TheirHand::Rock => 3,
            TheirHand::Paper => 1,
            TheirHand::Scissors => 2,
        }
    }

    score += hand_score;
    score
}

fn main() {
    let contents = get_data();
    let all_rounds = contents.split("\n").collect::<Vec<&str>>();
    let mut running_score: u32 = 0;
    let mut correct_running_score: u32 = 0;
    for round in all_rounds {
        let score: u32 = calculate_round_score(round);
        running_score += score;

        let correct_score: u32 = calculate_correct_round_score(round);
        correct_running_score += correct_score;
    }

    println!("Solution to part 1: {:?}", running_score);
    println!("Solution to part 2: {:?}", correct_running_score);
}
