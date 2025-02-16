mod tictactoe;
use tictactoe::{Game, GameStatus};

fn main() {
  let mut game = Game::new();
  println!("{}", game);

  for m in 0..=8 {
    if let Some(new_game) = game.make_move(m) {
      game = new_game;
      println!("{}", game);
      if game.game_status() != GameStatus::Ongoing { break; }
    }
  }
}
