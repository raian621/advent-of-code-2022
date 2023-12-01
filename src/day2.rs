use std::{fs::File, error::Error, collections::HashMap};

#[path = "input.rs"] mod input;

enum Choice {
  ROCK,
  PAPER,
  SCISSORS
}

enum GameResult {
  LOSE,
  DRAW,
  WIN
}

struct GameWithResult {
  opponent_choice: Choice,
  game_result: GameResult
}

struct Game {
  your_choice: Choice,
  opponent_choice: Choice,
}

pub fn solve(input_file_path: &str) {
  let input_file = match File::open(input_file_path) {
    Err(why) => { 
      println!("{}", why);
      return;
    },
    Ok(file_) => file_
  };
  
  let raw_input: Vec<String> = input::read(input_file);
  let games: Vec<Game> = convert_input(&raw_input).unwrap();

  println!("The score for the strategy guide is {}", calculate_score(&games));
  let games: Vec<Game> = convert_input_with_result(&raw_input).unwrap();

  println!("The score for the revised strategy guide is {}", calculate_score(&games));
}

fn choice_points(choice: &Choice) -> i64 {
  match choice {
    Choice::ROCK => 1,
    Choice::PAPER => 2,
    Choice::SCISSORS => 3,
  }
}

fn game_points(game: &Game) -> i64 {
  match game.opponent_choice {
    Choice::ROCK => match game.your_choice {
      Choice::ROCK => 3,
      Choice::PAPER => 6,
      Choice::SCISSORS => 0,
    },
    Choice::PAPER => match game.your_choice {
      Choice::ROCK => 0,
      Choice::PAPER => 3,
      Choice::SCISSORS => 6,
    },
    Choice::SCISSORS => match game.your_choice {
      Choice::ROCK => 6,
      Choice::PAPER => 0,
      Choice::SCISSORS => 3,
    }
  }
}

fn calculate_score(games: &Vec<Game>) -> i64 {
  let mut total_score: i64 = 0;

  for game in games {
    total_score += game_points(game) + choice_points(&game.your_choice)
  }

  total_score
}

fn convert_input(raw_input: &Vec<String>) -> Result<Vec<Game>, Box<dyn Error>> {
  let mut games: Vec<Game> = Vec::new(); 
  for game in raw_input {
    let choices: Vec<&str> = game.split(" ").collect();

    games.push(Game{ 
      your_choice: match choices[1] {
        "X" => Choice::ROCK,
        "Y" => Choice::PAPER,
        "Z" => Choice::SCISSORS,
        _ => return Err("invalid choice for player".into()),
      },
      opponent_choice: match choices[0] {
        "A" => Choice::ROCK,
        "B" => Choice::PAPER,
        "C" => Choice::SCISSORS,
        _ => return Err("invalid choice for opponent".into()),
      },
    });
  }

  Ok(games)
}

fn choose_move(game_with_result: &GameWithResult) -> Choice {
  match game_with_result.opponent_choice {
    Choice::ROCK => match game_with_result.game_result {
      GameResult::LOSE => Choice::SCISSORS,
      GameResult::DRAW => Choice::ROCK,
      GameResult::WIN => Choice::PAPER,
    },
    Choice::PAPER => match game_with_result.game_result {
      GameResult::LOSE => Choice::ROCK,
      GameResult::DRAW => Choice::PAPER,
      GameResult::WIN => Choice::SCISSORS,
    },
    Choice::SCISSORS => match game_with_result.game_result {
      GameResult::LOSE => Choice::PAPER,
      GameResult::DRAW => Choice::SCISSORS,
      GameResult::WIN => Choice::ROCK,
    },
  }
}

fn convert_input_with_result(raw_input: &Vec<String>) -> Result<Vec<Game>, Box<dyn Error>> {
  let mut games_with_result: Vec<GameWithResult> = Vec::new(); 
  for game in raw_input {
    let choices: Vec<&str> = game.split(" ").collect();

    games_with_result.push(GameWithResult{ 
      game_result: match choices[1] {
        "X" => GameResult::LOSE,
        "Y" => GameResult::DRAW,
        "Z" => GameResult::WIN,
        _ => return Err("invalid choice for opponent".into()),
      },
      opponent_choice: match choices[0] {
        "A" => Choice::ROCK,
        "B" => Choice::PAPER,
        "C" => Choice::SCISSORS,
        _ => return Err("invalid choice for opponent".into()),
      },
    });
  }

  let mut games: Vec<Game> = Vec::new();
  
  for game_result in games_with_result {
    games.push(Game { your_choice: choose_move(&game_result), opponent_choice: game_result.opponent_choice })
  }

  Ok(games)
}
